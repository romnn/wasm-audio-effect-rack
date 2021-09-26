#include <FastLED.h>
#define FASTLED_ALLOW_INTERRUPTS 0

#define SERIAL_BAUD 115200  // Baudrate, i need 900 * 60 = 54 000 bytes per second = 54 000 * 8 = 432 000 bits per second
#define SERIAL_BAUD_FASTER 500000
bool is_connected = false;
int32_t num_led_strips = 0;
uint64_t frame = 0;
uint32_t speed = 1;
uint32_t lastStepFrame = 0;

int8_t *data_pin = NULL;
int32_t *num_leds = NULL;
// int8_t *color_buffer = NULL;

#define MAX(a,b) ((a) > (b) ? (a) : (b))
#define MIN(a,b) ((a) < (b) ? (a) : (b))

#define MAX_LEDS 300
CRGB leds[MAX_LEDS];

enum Instruction {
  CONNECT = 0,
  ALREADY_CONNECTED = 1,
  INIT = 2,
  DATA = 3,
  ACK = 4,
  ERROR = 5,
};
typedef enum Instruction Instruction;

void setup() {
  // delay(1000);
  Serial.begin(SERIAL_BAUD);
  // Serial.print("Hello");
  // Serial.print(is_connected);
  // Wait until the arduino is connected to master
  // write_instruction(CONNECT);
  // FastLED.addLeds<WS2812, 5, GRB>(leds, 300);
  while(!is_connected) {
    // Serial.print("CONNECT (version 3)\n");
    write_instruction(CONNECT);
    wait_for_bytes_with_timeout(1, 1000);
    get_messages_from_serial();
  }
  // here, the actual is alreay connected message will be sent
  // get_messages_from_serial();
  // get_messages_from_serial();
  // write_instruction(ACK);
  // setupLEDs()
}

void loop() {
  // FastLED.setDither(false);
  // FastLED.setMaxPowerInVoltsAndMilliamps(5, 400);

  // it is important that the backend only sends data when we can receive it
  // beacuse fastled show disables the interrupts, we could miss serial messages and everthing will be dragon land
  // write_instruction(READY);
  get_messages_from_serial();
  // todo: animate and interpolate
  // todo: read in the speed
  // if (frame - lastStepFrame >= speed) {
  int32_t offset = 0;
  int32_t step_size = 3;
  for (int32_t strip = 0; strip < num_led_strips; strip++) {
    // for (int32_t led = 1; led < num_leds[strip]; led++) {
    for (int32_t led = num_leds[strip]-1; led > 0; led--) {
      // leds[offset + led-1] = leds[offset + led];
      if (frame - lastStepFrame >= speed) {
        // do a step
        leds[offset + led] = leds[MAX(0,offset + led - step_size)];
      } else {
        // interpolate between the values
        // but this only works if we have enough memory, which we dont :(
        // leds[offset + led] += (leds[offset + led] - leds[offset + led - 1]);
      }
    }
    // leds[offset + num_leds[strip] - 1] = CRGB(r,g,b);
    offset += num_leds[strip];
  }
  if (frame - lastStepFrame >= speed) {
    lastStepFrame = frame;
  }
  frame++;
  // read_instruction(); // this will be an ack
}

void setupLEDs() {
  int32_t offset = 0;
  for (int32_t strip = 0; strip < num_led_strips; strip++) {
    // FastLED.addLeds<WS2812, 3, GRB>(leds, offset, num_leds[strip]);
    FastLED.addLeds<WS2812, 5, GRB>(leds, 0, 300);
    // FastLED.addLeds<WS2812, 6, GRB>(leds, 0, 300);
    /* switch (data_pin[strip]) {            
      case 3: {
          FastLED.addLeds<WS2812, 3, GRB>(leds, offset, num_leds[strip]);
          break;
        }
      case 4: {
          FastLED.addLeds<WS2812, 4, GRB>(leds, offset, num_leds[strip]);
          break;
        }
      case 5: {
          FastLED.addLeds<WS2812, 5, GRB>(leds, offset, num_leds[strip]);
          break;
        }
      case 6: {
          FastLED.addLeds<WS2812, 6, GRB>(leds, offset, num_leds[strip]);
          break;
        }
      default:
          break;
    }
    */
    // offset += num_leds[strip];
  }
  FastLED.setCorrection(TypicalLEDStrip);
  fill_solid(leds, MAX_LEDS, CRGB::Black);
}

void get_messages_from_serial()
{
  if(Serial.available() > 0) {
    // The first byte received is the instruction
    Instruction instruction = read_instruction();
    if(instruction == CONNECT) {
      if(!is_connected) {
        is_connected = true;
        // write_instruction(ALREADY_CONNECTED);
        write_instruction(CONNECT);
      } else {
        // If we are already connected do not send CONNECT to avoid infinite loop
        write_instruction(ALREADY_CONNECTED);
      }
    } else if (instruction == ALREADY_CONNECTED) {
      is_connected = true;
    } else {
      switch(instruction) {
        case INIT: {
          if (data_pin != NULL) free(data_pin);
          if (num_leds != NULL) free(num_leds);

          // receive new init parameters
          num_led_strips = read_i32();
          write_i32(num_led_strips);
          data_pin = (int8_t*)malloc(num_led_strips * sizeof(int8_t));
          num_leds = (int32_t*)malloc(num_led_strips * sizeof(int32_t));
          for (int32_t strip = 0; strip < num_led_strips; strip++) {
            data_pin[strip] = read_i8();
            write_i8(data_pin[strip]);
            num_leds[strip] = read_i32();
            write_i32(num_leds[strip]);
            // we dont have enough memory for a big color buffer
            // color_buffer = (int8_t*)malloc(num_led_strips * num_leds[strip] * 3 *  sizeof(int8_t));
          };
          setupLEDs();
          break;
        }
        case DATA: {
          uint8_t r = read_u8();
          uint8_t g = read_u8();
          uint8_t b = read_u8();
          // fill_solid(leds, MAX_LEDS, CRGB(r,g,b));
          // put the first element in the last slot
          int32_t offset = 0;
          for (int32_t strip = 0; strip < num_led_strips; strip++) {
            for (int32_t led = 0; led < num_leds[strip]; led++) {
              // leds[offset + led-1] = leds[offset + led];
              // leds[offset + led] = CRGB(r,g,b);
            }
            leds[offset] = CRGB(r,g,b);
            offset += num_leds[strip];
          }
          // FastLED.setBrightness(10); // scale 0-255
          FastLED.show();
          break;
        }
        // unknown instruction
        default: {
          write_instruction(ERROR);
          return;
        }
      }
      write_instruction(ACK);
    }
  }
}

Instruction read_instruction() {
  return (Instruction) Serial.read();
}

void write_instruction(enum Instruction instruction) {
  uint8_t* instruction_code = (uint8_t*) &instruction;
  Serial.write(instruction_code, sizeof(uint8_t));
}

void write_i8(int8_t num) {
  Serial.write(num);
}

void write_u8(uint8_t num) {
  Serial.write(num);
}

void write_i16(int16_t value) {
  int8_t buffer[2] = {(int8_t) (value & 0xff), (int8_t) (value >> 8)};
  Serial.write((uint8_t*)&buffer, 2*sizeof(int8_t));
}

void write_i32(int32_t value) {
  int8_t buffer[4] = {(int8_t) (value & 0xff), (int8_t) (value >> 8 & 0xff), (int8_t) (value >> 16 & 0xff), (int8_t) (value >> 24 & 0xff)};
  Serial.write((uint8_t*)&buffer, 4*sizeof(int8_t));
}

int8_t read_i8() {
  wait_for_bytes(1);
  return (int8_t) Serial.read();
}

uint8_t read_u8() {
  wait_for_bytes(1);
  return (uint8_t) Serial.read();
}

int16_t read_i16() {
  int8_t buffer[2];
  wait_for_bytes(2);
  read_signed_bytes(buffer, 2);
  return (((int16_t) buffer[0]) & 0xff) | (((int16_t) buffer[1]) << 8 & 0xff00);
}

int32_t read_i32() {
  int8_t buffer[4];
  wait_for_bytes(4);
  read_signed_bytes(buffer, 4);
  return (((int32_t) buffer[0]) & 0xff) | (((int32_t) buffer[1]) << 8 & 0xff00) | (((int32_t) buffer[2]) << 16 & 0xff0000) | (((int32_t) buffer[3]) << 24 & 0xff000000);
}

void wait_for_bytes(int num_bytes) {
  // wait for incoming bytes
  while (Serial.available() < num_bytes) {
    // we busy wait here for the data
  }
}

void wait_for_bytes_with_timeout(int num_bytes, unsigned long timeout) {
  unsigned long startTime = millis();
  // wait for incoming bytes or exit if timeout
  while ((Serial.available() < num_bytes) && (millis() - startTime < timeout)) {
    // we busy wait here for the data
  }
}

void read_signed_bytes(int8_t* buffer, size_t n) {
  size_t i = 0;
  int c;
  while (i < n)
  {
    c = Serial.read();
    if (c < 0) break;
    *buffer++ = (int8_t) c; // buffer[i] = (int8_t)c;
    i++;
  }
}
