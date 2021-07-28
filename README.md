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
  - stream data from rust backend to frontend
  - implement base class and interface for effects
  - effect parameterizer chaining
  - select the current parameterizer via the controller page

- adriana
  - create a rust backend binary with a websocket server
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
- since we will be using rust and a lot of protobuf, we might as well just add support for building with bazel?


#### Done
