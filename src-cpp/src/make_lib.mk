OS := $(shell uname)

ifeq ($(OS), Darwin)  # macOS
	CXX = g++
	CC = clang++
	LLAMA_CPP_PATH = /Users/kamran/llama.cpp
	DY_TARGET=dylib
endif
ifeq ($(OS), Linux)
	CXX = g++
	CC = gcc
	LLAMA_CPP_PATH = /media/kamran/T7/llama.cpp
	DY_TARGET=so
endif

JSON_INCLUDE=include
LLAMA_INCLUDE=$(LLAMA_CPP_PATH)/include
GGML_INCLUDE=$(LLAMA_CPP_PATH)/ggml/include
LLAMA_LIBS=$(LLAMA_CPP_PATH)/build/bin

CXXFLAGS = -std=c++17 -Wall -I$(LLAMA_INCLUDE) -I$(GGML_INCLUDE) -I$(JSON_INCLUDE)
LDFLAGS = -L$(LLAMA_LIBS) -lllama -lggml -lggml-base 

TARGET_DIR = ../target
SRC = lib.cpp
TARGET = $(TARGET_DIR)/libmathapp.$(DY_TARGET)
STATIC_TARGET = $(TARGET_DIR)/libmathapp.a
OBJ = $(patsubst %.cpp,$(TARGET_DIR)/%.o,$(SRC))

all: $(TARGET_DIR) $(STATIC_TARGET) $(TARGET) 

$(TARGET_DIR):
	mkdir -p $@

$(TARGET): $(OBJ)
	$(CC) -shared -o $@ $^ $(LDFLAGS)
	@echo "dylib $@ created"

$(STATIC_TARGET): $(OBJ)
	@echo "Creating static library $@"
	ar rcs $@ $^
	@echo "Static library $@ created"

$(TARGET_DIR)/%.o: %.cpp | $(TARGET_DIR)
	$(CC) $(CXXFLAGS) -fPIC -c $< -o $@

clean:
	rm -f $(OBJ) $(TARGET) $(STATIC_TARGET)

.PHONY: all clean
