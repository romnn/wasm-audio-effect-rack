## WASM Effect Rack

This is very much a work in progress and only worked on for special occasions.

It might be very cool one day though...

#### Project structure

web
math - contains math, functions etc. - and a lot of
disco (uses viewer, controller, math)
viewer
controller
remote
launcher
docs
disco

- math - contains math, functions etc. - and a lot of
- hardware
- analysis
- proto

#### Org name options

discodiscodiscodisco
discodiscoorg
cheapdisco
diskoorg
disco-org
disko-org
diskodiskodisko
disko-project

#### docs and landing page site inspiration

https://after-dark.habd.as/feature/extended-builds/

#### NPM client name options

@disco/disco is the all in one application
@disco/core contains math, communication, p2p etc.
@disco/controller contains the ui and logic for the controller
@disco/viewer contains the ui and logic for the viewer
@disco/launcher contains the launcher application UI

#### Python client name options

pip install discodisco

pip install diskopy

pip install disko-py

pip install pydisko

pip install py-disko

pip install disco-client

pip install silentdisco

pip install remotedisco

#### Usage

Start the standalone application

```bash
invoke start-standalone
invoke build-standalone
invoke debug-build-standalone
```

#### Utils

For some common tasks, utility functions are provided

```bash
# convert any audio file to wav for easy streaming
invoke to-wav ./path/to/audio.mp3 ./path/to/output.wav
```

#### Connecting an arduino

Communication with an arduino microchip is currently implemented via serial connection. To check for a connected arduino and send instructions to it run the following:

```bash
# check if there is a device and check the permissions
ls -l /dev/ttyACM*

# if needed, change the permissions
sudo chown $(id -run) /dev/ttyACM0
```

#### TODO (ASAP)

- Upload pip package with org user so that the name is safe
- Upload NPM packages with org user so that the names are safe

#### TODO backend

- check for existing connections before doing anything else or even starting something
- move the map function to somewhere where it can be exposed to JS and python
- limit the amount of messages sent by the backend to be max 60fps
- make it so that the playback does not stop after one hour but actually wait for an event
- package the backend in an easy to install gui application for osx, linux and windows
  - allow auto update
  - do not use electron because it is such an overkill
- package wrapper libraries for python, node and rust
- expose all rust math util functions for use in the frontend via wasm

#### TODO text transform visualization

- allow parameterization of the color and position moving speed for each char
- move the position logic to the paramterizer
- animate parameter changes
- fix the bug with the scroll
- use HSL for generating colors
- implement a strobe mode
- implement a nice colorful background
- play with the energy and spectrum effects to try to come up with something
- overlay the spectrum based transform of text with weight center based approach
- make parameterized options in grpc so those should also be grpc based so that they can be synced
- add a custom orbiter that keeps more visible

#### TODO controller

- handle resize window event to also resize the webgl canvas
- change the tab titles based on the session and instance id
- create a very simple controller page that allows to select the current effect and shows the options for the current effect
- make the linter happy
- select the current parameterizer via the controller page

#### TODO proto

- use custom proto tags to define for all controls that should be able to be used via midi what kind of control they are and min and max values
  - otherwise, the controls will be overwritten when configuring handlers for midi events manually

#### done

- remove unnecessary stuff in rust and make the linter happy
- allow multiple connections to both the controllers and the viewers
- allow running multiple analyzers at the same time
  - analyzer should define its preferred buffer size and push its own updates
  - the real time requirement should be dependent on the buffer size
- allow the backend to have multiple connections open and terminate them after some amount of failed send events/ time
  - log how many people are currently connected
  - redirect every controller and viewer page to a custom instance url
  - only if the instance matches connections should be re-attached
- implement a pride mode
- rename subscribe/unsubscribe to connect/disconnect
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
