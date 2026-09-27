TARGET = build/firmware
CC = arm-none-eabi-gcc
CP = arm-none-eabi-objcopy
SZ = arm-none-eabi-size

CPU = -mcpu=cortex-m4 -mthumb -mfpu=fpv4-sp-d16 -mfloat-abi=hard
CFLAGS = $(CPU) -Wall -Wextra -Og -g3 -fdata-sections -ffunction-sections -Isrc
LDFLAGS = $(CPU) -Tlinker.ld -Wl,-Map=$(TARGET).map,--gc-sections

SRCS = src/main.c src/startup.c
OBJS = $(SRCS:.c=.o)

all: $(TARGET).elf $(TARGET).bin $(TARGET).hex
	$(SZ) $(TARGET).elf

$(TARGET).elf: $(OBJS) linker.ld
	@mkdir -p build
	$(CC) $(OBJS) $(LDFLAGS) -o $@

%.o: %.c
	$(CC) -c $(CFLAGS) $< -o $@

$(TARGET).bin: $(TARGET).elf
	$(CP) -O binary $< $@

$(TARGET).hex: $(TARGET).elf
	$(CP) -O ihex $< $@

clean:
	rm -rf build src/*.o

.PHONY: all clean
