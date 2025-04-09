# Initial Map and Robot Navigation

### Status: Accepted

## Context

We are building a simulation in Rust where robots explore a 2D grid map to locate different resources. The first implemenation will use a fixed map using the terminal, as the features are done an upgrade to something like ratataui Rust is planned.

Initial functionality includes:

- A map for the robot to navigate in

- A robot that can move on the map and find the resource

- Pathfinding for the robot using a Breadth-First Search (BFS) algorithm

A decision has to be made about how to structure the navigation logic and map representation to support future features like multiple robots on the same map, finding the resources and returning them to the base.

## Decision 

- Use a Breadth-First Search (BFS) algorithm for pathfinding:

- Keep rendering in the terminal using ANSI escape codes for simple visual output.

## Consequences

- Easy to understand and debug, ideal for early prototyping

- BFS is a good algorithm for our application

- Static maps make it easier to test layout and logic

- Refactoring will be required as more features are implemented

- Large maps or many concurrent robots might be problematic
    
- Map and resources is still hardcoded

## Next Steps

- Introduce support for multiple robots

- Add retrieve and bring back resources to base

- add randomly generated resources in the map

- Consider separating rendering logic from simulation logic for modularity

- Potentially explore A* algorithm for better performance