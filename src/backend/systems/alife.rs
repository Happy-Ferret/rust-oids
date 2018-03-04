use super::*;
use std::collections::HashMap;
use rand;
use core::geometry;
use core::clock::SimulationTimer;
use backend::obj;
use backend::obj::Transformable;
use backend::obj::Identified;
use backend::world;
use backend::world::gen;
use backend::world::agent;
use backend::world::segment;
use backend::world::AgentState;
use backend::world::alert;
use backend::world::AlertReceiver;
use serialize::base64::{self, ToBase64};
use super::messagebus::Outbox;

type StateMap = HashMap<obj::Id, agent::State>;
type GeneMap = HashMap<obj::Id, gen::Dna>;

pub struct AlifeSystem {
	dt: Seconds,
	simulation_timer: SimulationTimer,
	source: Box<[world::Feeder]>,
	eaten: StateMap,
	touched: GeneMap,
}

impl System for AlifeSystem {
	fn import(&mut self, world: &world::World) {
		self.source = world.feeders().to_vec().into_boxed_slice();
		self.eaten = Self::find_eaten_resources(
			&world.agents(agent::AgentType::Minion),
			&world.agents(agent::AgentType::Resource),
		);
		self.touched = Self::find_touched_spores(
			&world.agents(agent::AgentType::Minion),
			&world.agents(agent::AgentType::Spore),
		);
	}

	fn update(&mut self, _: &AgentState, dt: Seconds) {
		self.dt = dt;
		self.simulation_timer.tick(dt);
	}

	fn export(&self, world: &mut world::World, outbox: &Outbox) {
		Self::update_resources(
			self.dt,
			&self.simulation_timer,
			&mut world.agents_mut(agent::AgentType::Resource),
			&self.eaten,
		);

		let (spores, corpses) = Self::update_minions(
			self.dt,
			world.extent.clone(),
			&mut world.agents_mut(agent::AgentType::Minion),
			&self.eaten,
		);

		let (hatch, fertilised) = Self::update_spores(
			self.dt,
			&self.simulation_timer,
			&mut world.agents_mut(agent::AgentType::Spore),
			&self.touched,
		);

		for &(ref transform, ref dna) in spores.into_iter() {
			outbox.post(alert::Alert::NewSpore.into());
			world.new_spore(outbox, transform.clone(), dna);
		}

		for &(ref transform, ref dna) in hatch.into_iter() {
			outbox.post(alert::Alert::NewMinion.into());
			world.hatch_spore(outbox, transform.clone(), dna);
		}

		for &(ref transforms, ref dna) in corpses.into_iter() {
			outbox.post(alert::Alert::DieMinion.into());
			for transform in transforms.iter() {
				world.decay_to_resource(outbox, transform.clone(), dna);
			}
		}

		for _ in 0..fertilised {
			outbox.post(alert::Alert::DieMinion.into());
		}
	}
}

impl Default for AlifeSystem {
	fn default() -> Self {
		AlifeSystem {
			dt: Seconds::new(1. / 60.),
			simulation_timer: SimulationTimer::new(),
			source: Box::new([]),
			eaten: StateMap::new(),
			touched: GeneMap::new(),
		}
	}
}

impl AlifeSystem {
	fn find_eaten_resources(minions: &agent::AgentMap, resources: &agent::AgentMap) -> StateMap {
		let mut eaten = HashMap::new();
		for (_, agent) in minions.iter().filter(|&(_, a)| a.state.is_active()) {
			for segment in agent.segments.iter().filter(|&s| {
				s.flags.contains(segment::Flags::MOUTH)
			})
				{
					if let Some(key) = segment.state.last_touched {
						if let Some(&agent::Agent { ref state, .. }) = resources.get(&key.id()) {
							eaten.insert(key.id(), (*state).clone());
						}
					}
				}
		}
		eaten
	}

	fn find_touched_spores(minions: &agent::AgentMap, spores: &agent::AgentMap) -> GeneMap {
		let mut touched = HashMap::new();
		for (_, spore) in spores.iter().filter(|&(_, a)| {
			a.state.is_active() && !a.state.is_fertilised()
		})
			{
				for segment in spore.segments.iter() {
					if let Some(key) = segment.state.last_touched {
						if let Some(ref agent) = minions.get(&key.id()) {
							if agent.gender() != spore.gender() {
								touched.insert(key.id(), agent.dna().clone());
							}
						}
					}
				}
			}
		touched
	}

	fn update_minions(dt: Seconds, extent: geometry::Rect, minions: &mut agent::AgentMap, eaten: &StateMap)
					  -> (Box<[(geometry::Transform, gen::Dna)]>,
						  Box<[(Box<[geometry::Transform]>, gen::Dna)]>,
					  ) {
		let mut spawns = Vec::new();
		let mut corpses = Vec::new();
		for (_, agent) in minions.iter_mut() {
			if agent.state.is_active() {
				agent.state.reset_growth();
				let maturity = agent.segment(0).unwrap().state.maturity();
				if maturity < 1. { // just grow a bit
					let r = 0.1;
					if agent.state.consume_ratio(1. - r, r) {
						let growth = 1. + r;
						agent.state.grow_by(growth);
						let zero = agent.segment(0).unwrap().transform.position;
						for segment in agent.segments.iter_mut() {
							let maturity = segment.state.maturity();
							segment.state.set_maturity(maturity * growth);
							segment.transform.position = zero + (segment.transform.position - zero) * growth;
						}
					}
				} else { // reproduce if enough energy
					if agent.state.consume_ratio(0.95, 0.75) {
						spawns.push((
							agent.last_segment().transform().clone(),
							agent.dna().clone(),
						));
					}
				}
				for segment in agent.segments.iter_mut() {
					let p = segment.transform().position;
					if p.x < extent.min.x || p.x > extent.max.x || p.y < extent.min.y || p.y > extent.max.y {
						agent.state.die();
					}
					if segment.flags.contains(segment::Flags::MOUTH) {
						if let Some(id) = segment.state.last_touched {
							if let Some(eaten_state) = eaten.get(&id.id()) {
								let energy = eaten_state.energy();
								agent.state.absorb(energy);
							}
						}
					}
					agent.state.consume(
						dt.get() as f32 * segment.state.get_charge() * segment.growing_radius(),
					);
					segment.state.update(dt);
				}

				if agent.state.energy() < 1. {
					let transforms = agent.segments.into_iter()
						.map(|segment| segment.transform.clone())
						.collect::<Vec<_>>();
					corpses.push((transforms.into_boxed_slice(), agent.dna().clone()));
					agent.state.die();
				}

				if let Some(segment) = agent.first_segment(segment::Flags::TRACKER) {
					agent.state.track_position(&segment.transform.position);
				}
			}
		}
		(spawns.into_boxed_slice(), corpses.into_boxed_slice())
	}

	fn update_resources(dt: Seconds, timer: &SimulationTimer, resources: &mut agent::AgentMap, eaten: &StateMap) {
		for (_, agent) in resources.iter_mut() {
			if eaten.get(&agent.id()).is_some() {
				agent.state.die();
			} else if agent.state.energy() <= 0. {
				agent.state.die();
			} else if agent.state.lifecycle().is_expired(timer) {
				agent.state.die();
			} else if agent.state.is_active() {
				for segment in agent.segments.iter_mut() {
					segment.state.update(dt)
				}
			}
		}
	}

	fn crossover(dna: &gen::Dna, foreign_dna: &Option<gen::Dna>) -> gen::Dna {
		match foreign_dna {
			&Some(ref foreign) => {
				gen::Genome::new(&foreign)
					.crossover(&mut rand::thread_rng(), dna)
					.dna()
					.clone()
			}
			&None => dna.clone(),
		}
	}

	fn update_spores(dt: Seconds, timer: &SimulationTimer, spores: &mut agent::AgentMap, touched: &GeneMap)
					 -> (Box<[(geometry::Transform, gen::Dna)]>, usize) {
		let mut spawns = Vec::new();
		let mut fertilise_count = 0usize;
		for (spore_id, spore) in spores.iter_mut() {
			if spore.state.lifecycle().is_expired(timer) {
				spore.state.die();
				spawns.push((
					spore.transform().clone(),
					Self::crossover(spore.dna(), spore.state.foreign_dna()),
				))
			} else if spore.state.is_active() {
				for segment in spore.segments.iter_mut() {
					if let Some(key) = segment.state.last_touched {
						if let Some(touched_dna) = touched.get(&key.id()) {
							info!(
								"fertilised: {} by {} as {}",
								spore_id,
								key.id(),
								touched_dna.to_base64(base64::STANDARD)
							);
							fertilise_count += 1;
							spore.state.fertilise(touched_dna);
						}
					}
				}
				for segment in spore.segments.iter_mut() {
					segment.state.update(dt)
				}
			}
		}
		(spawns.into_boxed_slice(), fertilise_count)
	}
}
