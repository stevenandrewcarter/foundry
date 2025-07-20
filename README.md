# Foundry

> Build Infrastructure end to end.

Foundry aims to be a alternative solution to Infrastructure as Code tooling. The
main goals the project will attempt to provide is a rich base language, in this
case Rust using Rhia scripting engine. In addition, it will attempt to provide
a complete end to end solution, building the targets and performing configuration
management on the targets as well.

Foundry will provide a few concepts that will help a user deploy a solution on
different types of environments. The concepts are _forging_ and _tempering_, which
will be described in detail in the following sections.

## Why Rust?

Rust has gained a lot of popularity in recent years and has started to show itself
as the clear winner when it comes to performance and stability. When considering
IaC tooling performance may not always be a high concern, but stability is a 
different matter. Rust is a staticly compiled language, which means a scripting
engine would be preferred for the tool. IaC tools often feel like they need to 
execute in a iterative manner to build solutions, so removing any compile time
requirements is desired.

## What about Ansilbe / Terraform?

Ansible and Terraform are both IaC tools that focus on one aspect of a deployment
and provide great tooling for that aspect. Ansible is great at configuration
management, but is not pleasant for Infrastructure. Terraform has the opposite 
problem in being great at defining the Infrastructure, but not the configuration
management parts. Foundry will attempt to provide syntax and solutions for both
types of requirements.

## Foundry Concepts

The following sections looks into how Foundry attempts to provide and End to End
solution for Deployments.

### Forging

The



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
