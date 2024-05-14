import os

# Define the ANSI color codes
colors = [30, 31, 32, 33, 34, 35, 36, 37, 90, 91, 92, 93, 94, 95, 96, 97]

# Loop through each color
for color in colors:
    # Start the line with the color code
    line = f'\033[{color}m'
    
    # Loop through each ASCII character
    for i in range(256):
        # If the character is non-printable, add a '^'
        if i < 32 or i == 127:
            line += '^'
        else:
            # Otherwise, add the character itself
            line += chr(i)
    
    # End the line with the reset color code
    line += '\033[0m'
    
    # Print the line
    print(line)