#include <SFML/Graphics.hpp>
#include <iostream>
#include <filesystem>

int main()
{
    // Create the window
    sf::RenderWindow window(sf::VideoMode(800, 600), "SFML Image Example");

    // Load the image
    sf::Texture texture;
    if (!texture.loadFromFile("C:/Projects/weBallin/art/oss_rn.png"))
    {
        // Handle error if the image file cannot be loaded
        std::filesystem::path currentPath = std::filesystem::current_path();

        // Print the current working directory
        std::cout << "Current Working Directory: " << currentPath << std::endl;

    }

    // Create a sprite and set its texture
    sf::Sprite sprite(texture);

    sprite.setScale(16.0f, 8.0f);

    texture.setSmooth(true);

    // Run the program as long as the window is open
    while (window.isOpen())
    {
        // Check all the window's events that were triggered since the last iteration of the loop
        sf::Event event;
        while (window.pollEvent(event))
        {
            // "Close requested" event: close the window
            if (event.type == sf::Event::Closed)
            {
                window.close();
            }
        }

        // Clear the window with black color
        window.clear(sf::Color::Black);

        // Draw the sprite (your image)
        window.draw(sprite);

        // End the current frame
        window.display();
    }
    return 0;
}