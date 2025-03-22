CXX = g++
CC = clang++
CXXFLAGS = -std=c++17 -Wall -I/Users/kamran/llama.cpp/include -I/Users/kamran/llama.cpp/ggml/include
LDFLAGS = -L/Users/kamran/llama.cpp/build/bin -lllama -lggml -lggml-base 

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
	rm -rf $(TARGET_DIR)

.PHONY: all clean

