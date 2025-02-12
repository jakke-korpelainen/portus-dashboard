tailwind.config = {
  theme: {
    extend: {
      fontFamily: {
        'mono': ['"Anonymous Pro"', 'monospace'],
        'sans': ['"Lexend Deca"', 'sans-serif'],
      },
      textColor: {
        DEFAULT: '#ffffff',
        accent: '#FF6B4A'  // Accessible warm reddish-orange
      },
      borderColor: {
        accent: '#FF6B4A'
      },
      backgroundColor: {
        accent: '#FF6B4A',
        'dark-row': '#1a1a23'  // Slightly lighter than the #16161d background
      }
    }
  }
}