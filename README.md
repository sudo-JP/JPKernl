# RPKernl: A Bare-Metal Kernel for Raspberry Pi Pico

**RPKernl** is a custom, preemptive kernel written in Rust for the Raspberry Pi Pico (RP2040 microcontroller). This project is an exploration of low-level systems programming, focusing on process scheduling, context switching, and system call infrastructure in a `no_std` environment.

## Project Status
Active Development. Core scheduling and process management are implemented. System calls and advanced scheduling features are in progress.

## Features
- **Preemptive Scheduling:** Implements a Round-Robin (RR) scheduler.
- **Process Management:** Creates and manages Process Control Blocks (PCBs) with states (Ready, Running, Blocked).
- **Context Switching:** Dual-mode switching (cooperative and interrupt-driven) using hand-written ARM assembly.
- **System Call Interface:** Foundation for system calls, with a `sleep()` call implemented using a priority heap queue.
- **Hardware Interaction:** Abstracts RP2040 peripherals (GPIO, timers) via the `rp2040-hal` crate.

## Technical Overview
### Kernel Core
The kernel's central component is its preemptive scheduler. It manages a queue of ready processes and uses a timer interrupt to trigger context switches, ensuring fair CPU time allocation.

### Process and Memory
- **PCB Allocation:** PCBs are statically allocated in a dedicated memory section defined in `memory.x`.
- **State Management:** Tracks process state and metadata to facilitate scheduling decisions.

### Context Switching
Two switching methods are implemented:
1.  **Cooperative:** A process yields control voluntarily.
2.  **Interrupt-Driven:** A timer interrupt forces a switch. The switch logic saves/restores the ARM Cortex-M0+ hardware context using inline assembly (`naked_fn`).

### System Calls
The `sleep()` system call demonstrates the interface. Processes block for a duration and are managed via a min-heap priority queue that wakes them efficiently.

## Safety and `unsafe` Usage
This kernel necessarily uses `unsafe` Rust for direct hardware manipulation, raw pointer dereferencing, and assembly blocks.
