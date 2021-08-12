#define SERIAL_BAUD 115200  // Baudrate
bool is_connected = false;

enum Instruction {
  ANNOUNCE = 0,
  INIT = 1,
  DATA = 2,
  ACK = 3,
};
typedef enum Instruction Instruction;

void setup() {
  // put your setup code here, to run once:
  // Init Serial
  Serial.begin(SERIAL_BAUD);
  // Wait until the arduino is connected to master
  // Serial.print("Hello world.\n");
  // get_messages_from_serial();
  /* 
   while(!is_connected)
  {
    write_instruction(HELLO);
    wait_for_bytes(1, 1000);
    get_messages_from_serial();
  }
  */
}

void loop() {
  // put your main code here, to run repeatedly:
  // get_messages_from_serial();
}

void get_messages_from_serial()
{
  if(Serial.available() > 0) {
    // The first byte received is the instruction
    Instruction instruction = read_instruction();
  }
}

Instruction read_instruction()
{
  return (Instruction) Serial.read();
}

/*int8_t read_i8()
{
  wait_for_bytes(1, 100); // Wait for 1 byte with a timeout of 100 ms
  return (int8_t) Serial.read();
}

int16_t read_i16()
{
  int8_t buffer[2];
  wait_for_bytes(2, 100); // Wait for 2 bytes with a timeout of 100 ms
  read_signed_bytes(buffer, 2);
  return (((int16_t) buffer[0]) & 0xff) | (((int16_t) buffer[1]) << 8 & 0xff00);
}

int32_t read_i32()
{
  int8_t buffer[4];
  wait_for_bytes(4, 200); // Wait for 4 bytes with a timeout of 200 ms
  read_signed_bytes(buffer, 4);
  return (((int32_t) buffer[0]) & 0xff) | (((int32_t) buffer[1]) << 8 & 0xff00) | (((int32_t) buffer[2]) << 16 & 0xff0000) | (((int32_t) buffer[3]) << 24 & 0xff000000);
}

void read_signed_bytes(int8_t* buffer, size_t n)
{
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
*/
