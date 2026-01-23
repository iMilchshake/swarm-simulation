# Swarm Simulation

## Motivation 
- Idea: Roaming boids-like swarms of units that try to survive as long as possible
- Scaling: These units can grow in size, but should also be punished for growing (e.g. lower movement speed)
- Respawning: Every N seconds a new swarm joins the arena, if less than some number of swarms are active
- Regeneration: Beacons allow respawning 1-N new units, but swarms temporarily become vulnerable while doing so. 
- Attacker advantage: To balance this staying still should be a disadvantage in terms of fighting
- Camping: To prevent camping at beacons, they spawn at random locations and disappear once used 
- Vision Range: Swarms should be incentivized to naturally roam around. Give them limited vision range, so staying in motion is the best option
- Units: Maybe the units could be space ships :3 
- Combat: Lock on mechanic that scales with enemy movement speed, so so smaller swarms can "harass" larger swarms 
- Decision Making: A Swarm should have some set of parameters that control decision making, set randomly on spawn (e.g. run away, hover, engage wrt. enemy count)
