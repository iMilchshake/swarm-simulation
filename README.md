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

## Swarm Fleeing Behaviour Ideas 
- Implement one system that can consider multiple sources of repulsion (multiple enemies, walls, ...)
- Idea: Instead of trying to calclate the optimal target position directly using geometrical rules try a more natural repulsion-based approach 
- each source of repulsion gives their respective angle a negative weight, and all nearby angles get a decaying negative weight (normal distribution?) 
- This means that if there is only one source of repulsion, the opposing angle will have the highest weight (straight away)
- However, once multiple repulsions are added on top of each other, the highest weighted angle will automatically consider all repulsions 
- Also, repulsors can be weighted differently: (i) by class (e.g. weight players more than walls), but also by distance (balance nearer repulsors stronger) 
- Problem: This approach does not consider current movement speed. This is bad because swarms take time changing directions -> But our approach allows us to add our current movement direction as a repulsor aswell (repulse directions that require larger turns). We could not only consider direction, but the magnitude of the planned motion or maybe better: current velocity. to punish it even harder if we already have high movement speed.
- Problem 2: How to handle walls? Initial idea was shooting raycasts every N degree. but if we then take normal distribution and stack those, we easily end up with a migh higher weight than players -> idea: get all wall angles, stack normal distibution, then re-normalize to a max scale of wall repulsors. Consider this as "one" distribution (e.g. like one player), so it does not matter whether 1 or 20 wall repulsor positions are used.
