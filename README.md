> this library is experimental  if you want to be part of the design phase open a github issue here 
https://github.com/nebtex/holo, **stay tuned**

# Introduction

Holo is a platform independent ui framework that is being implemented in rust, the idea is to 
to reduce the amount of work needed to create ui interfaces, that looks good in every platform

holo is the first library that will use `the spiral`, and is part of a series of bigger projects around it,
the `holo` name comes from the `holographic principle`, that is the ultimate goal of this library want to reach,
`a way to describe universes graphically in a smart way abusing of lazy programming techniques`.

`spiral`, are convenient data structures for distributed system `cdrt alike`, they are immutable and can communicate
changes with deltas **binary buffers** that are mergeable, (deltas can be put in a linked list and read without interrupt other thread/process computations),
those deltas can be used in  derivates of computations and form a deeply nested deferential equations system,
where computations are chained using `chain rule` in calculus https://en.wikipedia.org/wiki/Chain_rule, 
the chains are formed with the futures crates, all joined together to create a  deferential stream of information. 

spiral are being implemented here https://github.com/nebtex/spiral, but they are still very experimental, one of the intents
of holo is to help me to understand `spiral` more, they support basic types, but also allow to create custom ones. 

for other part `the spiral` is a endless data structure create with spiral, `fractal a like`.  in that sense holo is a viewport to `the spiral`

`the spiral` an deep neural networks work very similar, but `the spiral` is a unbounded, infinite data structure.     

this library will cover 3 different aspect of user interfaces (holo ux, holo 2d, holo 3d)

- **first stage(at the moment)**:   allow people to create beautiful user interface, with the 
 `least amount of work possible`, this aspect is oriented to **real-time** `web-pages`, `mobile`, `documents`, `presentations`, `repl` (yeah realtime!!!),  even terminal and 
 other system that interact with users, Holo ux should  be able to be rendered on `Holo2d` and `Holo3D` too, when they are implemented. 
 
- **second stage**: allow people to write 2d scalable graphics with holo,  circles, triangles, polygons, etcs. Holo2D, should be able to be rendered on Holo3d

- **third stage**: write 3d universes :). 


# Holo UX

## Features

### smart interfaces

HoloUX, will allow people to create smart UX interfaces, that will try to look good in any device, 
declare user interaction and forklows that integrate seamless with any backend, and be platform independent.
the first targeted environment  are browser due they popularity, but it will be really easy to implement holo in other environments.

Component will always try to render the best way possible taking into account things like screen and platform limitations,
according to the context, capabilities and intentions
  
for example if a gallery don't found enough space to be rendered, it will collapse to a image, if the image can't found 
space, it will be rendered as icon that when is pressed, will try to find a window or pop up 
with enough space to  show the gallery, (the windows have a capability, and the gallery have a intention)
all in the right position and time, and taking into account the current context, 
if the end user is filling a form, the gallery will need to create a new tab(without focusing) to not interrupt the form flow,
if the gallery is in a terminal environment, it will show a link to a web-browser instead of a full icon. 

### infinite scaling and smart event buffering

holo can be run in multiples process at the same time using share nothing architecture, spiral are perfect for things like 
incremental computing and computation in the edge, so they same techologies will be available in your render environment

### a matryoshka of components

each component define render function according to the screen size implementing the `Render<Screen>` trait,
that will look similar to react-like frameworks, also  each component can define it self in terms of other components,
all the component that it want , using the `RenderInto<T>` trait,  from most to least preferred component (in general or platform/theme specific).

so a component can have a list of optional render component whom at the same time have another list of optional components, 
whom .............. * 100, and their option have more options  Ad infinitum, this creates a large graph,
which only limitation imposed by the holo compiler will be to be acyclic.

your backend only need to chose few frontend components, and holo will walk the spiral and give you the perfect combination for every platform 

### streamline machine learning, ab/testing and online learning.

with holo you will be able to segment your audience in realtime, using behavioral techniques or whatever you prefer, and in top
of that ab/testing

### real-time screen sharing

realtime screen sharing will be a problem of the past with holo

### for web-browser render (static pages without javascript) use machine learning

due that the library does not use any allocation, it can be used in servers for rendiring your static website with minimal cost

## what holo_ux will focus the next months

1. browsers at the moment (initially only chrome)

2. perfect rendering in the mayor platforms

3. dynamic rendering, which means, that the optimization algorithms will run in the browsers

4. `events`/`intentions`/`capabilities` protocols design and implementation, that will allow components to talk each to others,
   example if there is stack showing cities, and a search control in top of it,
   the search control will gain the capability to search and filter by cities.
   
5. security policy applied to components, so limit the communication between them

6. use 0 allocations, this is still in research, but holo will use 0 allocations, from the backend to the client,
 thanks to async and  spiral programing, so the `allocation crate` is banned in this library at the moment.  
 people will still be able to create their own allocation in order to cache. but it will be extremely predictable

7. `on enter`, `on leave`, `mouseover`,  `focus`, events and animation, only ux animations, other kind of animation, like
props animations should be done in the `spiral`.  


## some holo_ux  long term focus but not main at the moment 

1. static rendering, let run holo render in multiples devices, browsers at the same time in parallel
   an pick the best css policies that match your spec using machine learning for every device, and render your site statically. 
   
2. run a/b testing with online learning, let your ux be optimal to every segment of the population, real time adaptation to world event, 
   seasons, etc. 

3. holo_ux should render in `holo2d` and `holo3d`, use your same ux of your mobile inside your games, without do anything


## what holo_ux will not allow 


1. define containers sizes and padding manually (there will be multiples algorithms, that will do this for you),
2. let people chose font sizes/ line height, or any other type of font configuration, different to font-family.
3. colors, backgrounds, etc, all those thing depend of each platform and should be handle there using `theming`

which means **any kind of manual work  is not desirable**

## DOm

the holo doom is in a design phase 

![alt text](../holodom.jp)

