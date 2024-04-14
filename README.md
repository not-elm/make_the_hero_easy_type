# make the hero

This game have been created for the demonstration of [`bevy_flurx`](https://github.com/not-elm/bevy_flurx).

I would be happy if I could provide solutions to questions such as how to write the game's process flow,
and how to use a `Reactor`.

## Game rules

The condition for stage clear is to ensure that
only one cell with the same number as `goal` exists that on the stage.

All cells can be moved only once.
If another cell exists at the destination, `calc` or `swap` operation is performed on the two cells.
which operation is performed depends on the direction of movement.

- The direction `calc` performed: `left up`, `right up`, `right down` and `left down`
- The direction `swap` performed: `left`, `up`, `right`, and `down`

### calc

`calc` uses two cells, applies four arithmetic operations that differ according to the direction of movement ,
and create the new cell has the number with resulting number.

The new cell will be placed at the move destination and the two old cells will be deleted.

```
move dist = md
move source = ms
```

| move direction                        | operation               |  
|---------------------------------------|-------------------------|
| left up                               | md + ms                 |
| right up                              | md - ms                 |
| left down                             | md * ms                 | 
| right down(if ms is zero, can't move) | md / ms                 | 

### swap

`swap` swaps `md` and `ms`.

For important point, `ms` is determined to have moved, but `md` is determined not.

## Controls

| key or mouse | operation            | 
|--------------|----------------------|
| `R`          | retry this stage     | 
| `G`          | generate a new stage |
| `P`          | play answer          | 


## Build and run from source code

> [!NOTE]
> Since I do not put up any songs on GitHub,
> please run it with default feature when you run it from the source code.
