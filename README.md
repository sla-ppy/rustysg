# rustysg

## Metadata

Each template file may start with some metadata:

```yaml
---
title: "My Post"
date: "2020-01-01"
time: "12:30:00"
description: "This is my first post"
author: "John Doe"
---
<h1>{{ title }}</h1>
<p>My first post on this blog. Today I want to ...</p>
...
```

## Development

To work on this without having to constantly `cd` around, you can use the current working directory:

1. `cargo run -- create .`
2. `cargo run -- generate .`
