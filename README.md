<p align="center">
  <h1 align="center">RabbiTui</h1>
  <p align="center">A terminal client for the RabbitMQ Management API</p>
</p>

> [!IMPORTANT]  
> Please note that the rabbitui project is still in an early stage, and many 
> features are still missing. Please check our 
> [roadmap](https://github.com/orgs/coding-kelps/projects/11) to learn about our 
> current progress and upcoming features.

## Overview

This repository host a fork of @maxmindlin's [rabbitui](https://github.com/maxmindlin/rabbitui).

### Why fork this project?

We decided to fork @maxmindlin's repository because it appears to be no longer actively maintained.

Additionally, we plan to expand the TUI's platform compatibility to support more environments.

### Usage

There are a number of cli options for starting up the UI. For help use `rabbitui --help`.

By default, it connects to `http://localhost:15672` with the default credentials. You can change this via cli parameters.

At any time in the application you can press `?` to see a help menu for the panel you are in.
