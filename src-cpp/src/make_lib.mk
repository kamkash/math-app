CXX = g++
CC = clang++
CXXFLAGS = -std=c++17 -Wall -I/Users/kamran/llama.cpp/include -I/Users/kamran/llama.cpp/ggml/include
LDFLAGS = -L/Users/kamran/llama.cpp/build/bin -lllama -lggml -lggml-base 

TARGET_DIR = ../target
SRC = lib.cpp
TARGET = $(TARGET_DIR)/libmathapp.dylib
STATIC_TARGET = $(TARGET_DIR)/libmathapp.a
OBJ = $(patsubst %.cpp,$(TARGET_DIR)/%.o,$(SRC))

all: $(TARGET_DIR) $(STATIC_TARGET) $(TARGET) 

$(TARGET_DIR):
	mkdir -p $@

$(TARGET): $(OBJ)
	$(CC) -shared -o $@ $^ $(LDFLAGS)

$(STATIC_TARGET): $(OBJ)
	@echo "Creating static library $@"
	ar rcs $@ $^
	@echo "Static library $@ created"

$(TARGET_DIR)/%.o: %.cpp | $(TARGET_DIR)
	$(CC) $(CXXFLAGS) -fPIC -c $< -o $@

clean:
	rm -f $(OBJ) $(TARGET) $(STATIC_TARGET)

.PHONY: all clean
