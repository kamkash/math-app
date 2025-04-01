// Example usage: main.cpp
#include "libllama-chat.h"
#include <iostream>

nlohmann::json get_weather(const nlohmann::json& arguments) {
    // Simulate a weather API call.
    std::string city = arguments["city"].get<std::string>();
    nlohmann::json weather_data = {
        {"city", city},
        {"temperature", 25.0},
        {"condition", "Sunny"}
    };
    return weather_data;
}

int main() {
    LlamaChat chat_bot("path/to/your/model.gguf"); // Replace with your model path

    std::string response = chat_bot.chat("Hello, how are you?");
    std::cout << "Chat Response: " << response << std::endl;

    nlohmann::json structured_data = chat_bot.generateStructuredOutput("Extract name and age from this text: 'John is 30 years old.'");
    std::cout << "Structured Output: " << structured_data.dump(4) << std::endl;

    chat_bot.registerFunction("get_weather", get_weather);

    nlohmann::json function_args = {{"city", "London"}};
    nlohmann::json function_result = chat_bot.callFunction("get_weather", function_args);
    std::cout << "Function Result: " << function_result.dump(4) << std::endl;

    return 0;
}