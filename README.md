# Foundry

> Spawn a worker that can complete a single task well, and then it will disappear.

Rust Project for running automation tools (Ansible, Terraform, etc.). With an additional Rust based version that will
perform actions against a remote host as well.

The Project will be split into two components. The first component is a Task Runner that will execute Tasks against a
configured remote instance. The Tasks will be simple to start with, but can perform more complex structures by using
IaC constructs to execute.

The second component is a Task Runner that will execute the Tasks as needed.

Use Rhia or Rune as the execution engine for the Rust Scripts. This will allow a library of Task runners to be available
to a "Rust Script".

## Configure for Rust

The following commands should be executed in order to start developing the project

```shell
sudo apt-get install gcc
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Further References

https://howtorust.com/implementing-an-application-container-in-rust/