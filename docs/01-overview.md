# Overview

The daemon works with two concepts: **services** and a **pipeline**. Services can be though of as any program that might run on the car and a pipeline is a colleection of those services that get started and stopped together. The definition of a pipeline is a list of enabled services. In the case of roverd, the pipeline is **always** valid (empty pipelines are technically valid). This make it easier to reason about the state since we know that at any given time the stored pipeline (in `/etc/roverd/rover.yaml`) is always a valid one.

The following shows the three states of a pipeline: Empty, Startable and Started. From the Empty state one can set a pipeline. If that pipeline is invalid, it will be rejected an we remain in the empty state. On the other hand, if it is valid, then we transition to the Startable state from where we can start the rover. From this state any changes made to the pipeline will be checked again so if a new pipeline is invalid, it will be sent back to the Empty state.

![Pipeline States](https://github.com/user-attachments/assets/56cba2f5-cd62-4366-97b4-159fc9837299)

After starting the rover from the Startable state, the pipeline moves to the Started state. From there, if any process from a service exits, all other processes will be terminated and we are back in the Startable state. The stop command will similarly terminate all processes and bring us back to the Startable state.
