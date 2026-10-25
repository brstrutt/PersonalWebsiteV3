---
title: "Pixel Perfect Website"
date: 2025-11-29 18:11:29
lastmod: 2025-11-29 19:32:45
tags:
- Pixel Art
- HTML
- CSS
- Frontend
---

## Background

I'm getting a little tired of how this website looks. It needs an UPGRADE. With PIXEL ART!

Unfortunately pixel art looks best when all pixels are the same size, and crisp. And I've heard that getting crisp pixels in a browser can sometimes be...challenging.

So, in order to investigate WITHOUT completely trashing this website, I need a different website, just to mess around and investigate what kind of pixel perfect shenanigans I can get up to.

## The Playground

I've created [https://brstrutt.github.io/PixelArtPlayground/](https://brstrutt.github.io/PixelArtPlayground/) as a place for pixel art experiments.

If I'm lucky I'll remember to update this document whenever I try a new experiment.

## Experiments

### Pixel perfect text

To achieve pixel perfect text I did the following:

1. Grab a pixel art font from somewhere (I used [Able 5 by Stormgold](https://stormgold.itch.io/able-font))
2. Add the font to the website

    ```css
    @font-face {
        font-family: DefaultPixelFont;
        src: url(./assets/fonts/Able_5.ttf);
    }

    * {
        font-family: DefaultPixelFont;
    }
    ```

3. Investigate how many pixels tall the font ACTUALLY is (this one is 9 pixels tall despite claiming to be 5 pixels tall. 3 of those pixels are invisible borders at the top and bottom)
4. Set the `font-size` to a size that results in pixel perfect display (it must be a multiple of the pixel height of the font)

    ```css
    * {
        font-family: DefaultPixelFont;
        font-size: 18px; /* I choose 18px so each pixel in the font is a 2px by 2px square */
    }
    ```

Now you just need to make sure that ALL padding/margins/other things that can move the position of a div use pixel coords.

### Center a div in a pixel perfect way

To center my text in a pixel perfect way you can't rely on the browsers normal automatic centering behaviour. This could easily place the text on a partial pixel boundary, making it look BLURRY.

I centered it by calculating a pixel perfect width, and pixel perfect margins, based on the screen width. Applying those to the div effectively centered it.

1. The basic idea is to set the width of the div to 50% screen size, then add margins of 25% to each side:

    ```css
    main {
        background-color: #a9a9a9;
        width: calc(100vw / 2px);
        margin: calc(100vw / 4px);
    }
    ```

2. This unfortunately results in blurred text if the viewport width isn't perfectly divisible by 4....because 100v4/4px could return 120.5px
3. To fix this, we calculate the remainder of 100vw/4, and subtract that from the 100vw. This results in a value that is guaranteed to be divisible by 4:

    ```css
    main {
        background-color: #a9a9a9;
        width: calc(100vw / 2px);
        --viewWidthRemainderFour: mod(100vw, 4px);
        margin: calc((100vw - var(--viewWidthRemainderFour)) / 4px);
    }
    ```

4. This fixes the text in the div, but the issue still affects the width calculation, so we fix that too:

    ```css
    main {
        background-color: #a9a9a9;
        --viewWidthRemainderTwo: mod(100vw, 2px);
        width: calc(var((100vw - var(--viewWidthRemainderTwo))) / 2px);
        --viewWidthRemainderFour: mod(100vw, 4px);
        margin: calc((100vw - var(--viewWidthRemainderFour)) / 4px);
    }
    ```

5. And finally, stop centering it vertically using the viewWidth. Lets just set a constant vertical margin instead:

    ```css
    main {
        background-color: #a9a9a9;
        --viewWidthRemainderTwo: mod(100vw, 2px);
        width: calc(var((100vw - var(--viewWidthRemainderTwo))) / 2px);
        --viewWidthRemainderFour: mod(100vw, 4px);
        margin: 20px calc((100vw - var(--viewWidthRemainderFour)) / 4px);
    }
    ```

Perfect. Now just remember to use ONLY pixel coordinates when adding any padding or transformations or other such nonsense.

### CSS Pixel Art

I didn't need to do this...but I did.

I found [this css-tricks article](https://css-tricks.com/fun-times-css-pixel-art/) about CSS pixel art, and decided to try it.

So far I've gone with the following approach:

1. Create a CSS class that represents a single pixel on screen
2. Add a box shadow to that class with multiple coloured shadows. Each shadow is a single pixel.
3. Profit (by spending half an hour manually specifying individual pixels.....I should really automate this process if I want to keep doing this) **update:** You can use [https://pixelartcss.com](https://pixelartcss.com) to do this for you!
4. (Bonus points) Make the main css class match the full size of the pixel art, then move the class you created above to the `:after` element. This means you can right click the pixel art to get to the correct element, and it forces other elements out of the way.

The resulting css is an example css class you can just drop into a page to see a little pixel art of bellsprout:
{{< rawhtml >}}
<div class='bellsprout'></div>
{{</ rawhtml >}}

```css
.bellsprout {
    position: relative;
    width: 32px;
    height: 32px;
}

.bellsprout:after {
    position: absolute;
    content: "";
    /* Size of a single pixel. Box-shadow matches these dimensions for each shadow */
    width: 2px;
    height: 2px;

    /* Setup a colour palette to make life easier */
    --black: #000000;
    --yellow: #fbf236;
    --darkYellow: #85974a;
    --lightBrown: #8f563b;
    --brown: #7a4736;
    --darkBrown: #663931;
    --lightGreen: #99e550;
    --green: #6abe30;
    --darkGreen: #4b692f;
    --red: #d95763;
    --darkRed: #45283c;
    --shadow: #00000066;

    /* Set the initial pixel all other pixels are relative to */
    background: var(--yellow);
    top: 2px;
    left: 16px;

    box-shadow:
        2px 0px var(--yellow),
        4px 0px var(--yellow),
        -2px 2px var(--darkYellow),
        0px 2px var(--yellow),
        2px 2px var(--yellow),
        4px 2px var(--yellow),
        6px 2px var(--black),
        -4px 4px var(--lightBrown),
        0px 4px var(--darkYellow),
        2px 4px var(--black),
        4px 4px var(--yellow),
        6px 4px var(--yellow),
        -4px 6px var(--brown),
        2px 6px var(--darkYellow),
        4px 6px var(--yellow),
        6px 6px var(--red),
        8px 6px var(--red),
        -10px 8px var(--lightGreen),
        -8px 8px var(--green),
        -4px 8px var(--darkBrown),
        4px 8px var(--red),
        6px 8px var(--darkRed),
        -12px 10px var(--lightGreen),
        -10px 10px var(--green),
        -8px 10px var(--darkGreen),
        -6px 10px var(--darkGreen),
        -2px 10px var(--lightBrown),
        0px 10px var(--green),
        2px 10px var(--darkGreen),
        4px 10px var(--lightGreen),
        6px 10px var(--lightGreen),
        -14px 12px var(--green),
        -12px 12px var(--darkGreen),
        -10px 12px var(--darkGreen),
        -8px 12px var(--darkGreen),
        -2px 12px var(--darkBrown),
        2px 12px var(--darkGreen),
        4px 12px var(--darkGreen),
        6px 12px var(--lightGreen),
        8px 12px var(--green),
        0px 14px var(--lightBrown),
        4px 14px var(--darkGreen),
        6px 14px var(--green),
        0px 16px var(--brown),
        4px 16px var(--darkBrown),
        6px 16px var(--brown),
        -6px 18px var(--brown),
        0px 18px var(--lightBrown),
        2px 18px var(--darkBrown),
        8px 18px var(--darkBrown),
        -4px 20px var(--brown),
        -2px 20px var(--lightBrown),
        -4px 26px var(--shadow),
        0px 26px var(--shadow),
        2px 26px var(--shadow),
        6px 26px var(--shadow)
}
```

Use this by simply adding `<div class='bellsprout'></div>` to the HTML.
