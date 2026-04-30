OS := $(shell uname)
CXX = g++
CC = clang++


ifeq ($(OS), Darwin)
    LLAMA_CPP_PATH = /Users/kamran/mathappws/llama.cpp
	LLAMA_LIBS=$(LLAMA_CPP_PATH)/build-cpu/bin
    DY_TARGET=dylib
    # Add these frameworks for macOS GPU support
	LDFLAGS = -L$(LLAMA_LIBS) -lllama -lggml -lggml-base -lggml-metal \
	          -framework Foundation -framework Metal -framework MetalKit -framework Accelerate \
	          -Wl,-rpath,$(LLAMA_LIBS)
endif
ifeq ($(OS), Linux)
	LLAMA_CPP_PATH = /media/kamran/T7/llama.cpp
	LLAMA_LIBS=$(LLAMA_CPP_PATH)/build-cpu/bin
	DY_TARGET=so
	LDFLAGS = -L$(LLAMA_LIBS) -lllama -lggml -lggml-base -Wl,-rpath,$(LLAMA_LIBS)
endif

LLAMA_INCLUDE=$(LLAMA_CPP_PATH)/include
GGML_INCLUDE=$(LLAMA_CPP_PATH)/ggml/include
CXXFLAGS = -std=c++17 -Wall -I$(LLAMA_INCLUDE) -I$(GGML_INCLUDE)

TARGET_DIR = ../target
SRC = main.cpp
TARGET = $(TARGET_DIR)/main
OBJ = $(patsubst %.cpp,$(TARGET_DIR)/%.o,$(SRC))

all: $(TARGET_DIR) $(TARGET)

$(TARGET_DIR):
	mkdir -p $@

$(TARGET): $(OBJ)
	$(CC) -o $@ $^ $(LDFLAGS)

$(TARGET_DIR)/%.o: %.cpp | $(TARGET_DIR)
	$(CC) $(CXXFLAGS) -c $< -o $@

clean:
	rm -f $(OBJ) $(TARGET) 

.PHONY: all clean

