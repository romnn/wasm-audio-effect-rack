## WASM Effect Rack

This is very much a work in progress and only worked on for special occasions.

It might be very cool one day though...

#### Utils

For some common tasks, utility functions are provided

```bash
# convert any audio file to wav for easy streaming
invoke to-wav ./path/to/audio.mp3 ./path/to/output.wav
```

#### TODO

- goal for today
  - animate parameter changes
  - implement a pride mode
  - use HSL for generating colors
  - implement a strobe mode
  - iplement a nice colorful background
  - play with the energy and spectrum effects to try to come up with something
  - overlay the spectrum based transform of text with weight center based approach
  - make it so that the playback does not stop after one hour but actually wait for an event
  - make parameterized options in grpc so those should also be grpc based so that they can be synced
  - allow the backend to have multiple connections open and terminate them after some amount of failed send events/ time
    - log how many people are currently connected
    - redirect every controller and viewer page to a custom instance url
    - only if the instance matches connections should be re-attached
  - create a very simple controller page that allows to select the current effect and shows the options for the current effect
  - allow running multiple analyzers at the same time
    - analyzer should define its preferred buffer size and push its own updates
    - the real time requirement should be dependent on the buffer size
  - allow parameterization of the color and position moving speed for each char
  - fix the bug with the scroll
  - limit the amount of messages sent by the backend to be max 60fps
  - add a custom orbiter that keeps more visible
  - make the frontend linter happy
  - remove unnecessary stuff in rust and make the linter happy
  - allow multiple connections to both the controllers and the viewers
  - select the current parameterizer via the controller page

done

  - add parameterizer options as well
    - for ttf: exponential, linear, eased, or no fadeout
- pass the cli config object to the server instance so that it can use the cli arguments
- remove the requirement to have the debug flag in the params
  - debug, show fps and show controls should be part of the class
  - such that they can be changed via the controller
- create a font gallery class to select fonts dynamically
- try out the SOPHIE text and font <3
- query the audio inputs and try to get an live audio feed working
- move the whole match gaussian stuff to utils
- compute the fadeout factor using log and the resolution
- make the ttf vis look nicer with blur
- make weights change based on the mels
- allow different depths for different chars
- inject the token using interceptors so i dont always forget in the api calls
- create a visualization gallery that keeps a list of al the available visualizations
- randomly generate a token
- split the viewer and controller grpc services
- stream data from rust backend to frontend
- implement base class and interface for effects
- effect parameterizer chaining
- allow to specify the token via the url

- adriana

  - add a second thread with a very simple audio analyzer that just streams the audio volume to the frontend via websocket as a grpc message
  - use another temporary circular buffer for the width and for the color that is modified in the render loop based on the speed
  - only update the width targets once in a while and then gradually increase towards them based on the speed
  - include the groups bouding box
  - scale the camera to properly include all the text

- what i need for the party

  - fog machine
  - maybe a projector if gera is not there
  - dj controller audio monitor setup
  -

- check for memory leaks with valgrind by using nightly and the default allocator
- implement a heartbeat that checks when to disconnect
- lint because i cannot see all the warnings anymore
- add text based visualization with some warping and some parameters ready to be animated
- refactor the frontend visualization code to share a common interface for connecting with audio analysis results
- implement a generic websocket handler that can receive any GRPC control message s
- extend the router to render a controller page that interacts with a backend using grpc web and or websockets
- test out some audio analysis techniques using python for fast prototyping and send it to the frontend via websockets and grpc for python
- implement reading of audio signals in rust and check how good compatibility is in general
- read the arturia midi board to check what can be done with the web audio midi api
- implement remote control via p2p such that the recorder and the controller do not have to be in the same network even
  - generate qr codes to scan on mobile that redirect to the web controller with the token so that controlling can be done easily

#### Done

- create a rust backend binary with a websocket server
- since we will be using rust and a lot of protobuf, we might as well just add support for building with bazel?
