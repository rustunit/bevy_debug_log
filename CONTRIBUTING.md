# Contributing to bevy_debug_log

We happily welcome folks to contribute in some way. This document will help you in contributing, however you'll do that.

## Reporting issues and requesting features

Please report issues and request features at the relevant sections of this repository. For issues please state what you expected to happen, what went wrong, and any other extra info that you feel is relevant. For PRs please state the objective, how you are solving that objective with this PR, and how you have tested it / how reviewers may test it to confirm it works.

Check for duplicates before submitting, please.

## Testing

To test `bevy_debug_log` in the same way that the continuous integration does, run:

`just ci` 

This will check to be sure that your code runs, hasn't broken anything, has adequate documentation, and is formatted to our standards.

> [!NOTE]
> To install `just` for your system, see the [Just Programmer's Manual](https://just.systems/man/en/chapter_1.html).
