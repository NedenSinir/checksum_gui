import { extendTheme } from "@chakra-ui/react";

const theme = extendTheme({
  colors: {
    main: "#1f1e48",
    mainDark: "#313049",
    themeOrange: "#DD6B20",
    themeBlue: "#00bcd4",
    themeBlueDark: "#007399",
    myGradient: "linear(to-r, teal.400, teal.500, teal.600)",
  },

  styles: {
    global: {
      body: {
        textColor: "white", // Set the text color to white
      },
    },
  },

  components: {
    Switch: {
      baseStyle: {
        track: {
          bg: "gray.600",
          _checked: {
            bg: "themeOrange",
          },
        },
        thumb: {
          bg: "teal.300",
        },
      },
    },
    Modal: {
      baseStyle: {
        overlay: {
          bg: "blackAlpha.700", //change the background
        },
        dialog: {
          borderRadius: "md",
          borderColor: "themeOrange",
          bg: `main`,
        },
        body: {
          textColor: "white",
        },
      },
    },
    Tabs: {
      //doca bak https://chakra-ui.com/docs/components/tabs/theming#customizing-the-default-theme

      baseStyle: {
        tab: {
          _selected: {
            color: "white",
            bgGradient: "linear(to-r, teal.400, teal.500, teal.600)",
            borderBottomRadius: "0",
            borderTopRadius: "1rem",
          },
          _hover: {
            color: "white",
          },
        },
      },
    },
    Card: {
      baseStyle: {
        container: {
          borderRadius: "5px",
          borderWidth: "2px",
          borderColor: "themeOrange",
          textColor: "white",
        },
        footer: {
          borderTopWidth: "2px",
          borderColor: "themeOrange",
          bgColor: "main",
        },
        body: {
          borderRaidus: "0",
          borderWidth: "0px",
        },
      },
    },
    Link: {
      baseStyle: {
        _hover: {
          textDecoration: "underline",
        },
      },
    },

  },
});

export default theme;
