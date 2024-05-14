# Define the ANSI color codes for background and foreground
bg_colors = [40, 41, 42, 43, 44, 45, 46, 47, 100, 101, 102, 103, 104, 105, 106, 107]
fg_colors = [30, 31, 32, 33, 34, 35, 36, 37, 90, 91, 92, 93, 94, 95, 96, 97]

# Loop through each background color
for bg_color in bg_colors:
    # Loop through each foreground color
    for fg_color in fg_colors:
        # Skip if the foreground color is the same as the background color
        if bg_color % 10 == fg_color % 10:
            continue

        # Start the line with the color code
        line = f'\033[{fg_color};{bg_color}m'
        
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