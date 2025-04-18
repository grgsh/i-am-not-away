# i-am-not-away

[![Crates.io](https://img.shields.io/crates/v/i-am-not-away.svg)](https://crates.io/crates/i-am-not-away)
[![Documentation](https://docs.rs/i-am-not-away/badge.svg)](https://docs.rs/i-am-not-away)

## Overview

`i-am-not-away` is a Rust crate designed to prevent communication applications from automatically setting the "away" status when the user is not actively interacting with their keyboard. This crate aims to address the issue of being marked as "away" while engaged in activities such as reading a book, where keyboard interaction is minimal.

## Problem Statement

The initial approach was to simulate mouse movement to keep the system awake and prevent the "away" status from being triggered. An oscillator method was implemented to generate numbers that would be used to move the mouse cursor programmatically. However, it was discovered that simple mouse movements alone are insufficient to prevent other processes from detecting the "away" status.

## Current Status

As of now, the functionality to effectively prevent the "away" status is not fully operational. Further research is needed to identify additional actions that may be required to achieve the desired outcome. Suggestions and contributions towards finding a viable solution are welcome.

## Features

- Simulates mouse movement to prevent "away" status (currently in development).
- Research and exploration of additional methods to maintain active status.
