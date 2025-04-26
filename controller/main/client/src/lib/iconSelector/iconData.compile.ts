// Parse the Font Awesome icon metadata into a simple list

import metadata from "./icons.json";

// For some reason, 
export const icons = Object.entries(metadata)
    .filter(([_, value]) => value.styles[0] === "solid")
    .map(([key, value]) => {
        const unicodeString = String.fromCodePoint(parseInt(value.unicode, 16));
        return {
            name: key,
            searchTerms: value.search.terms.map(term => term.toLowerCase()),
            prefix: value.styles[0],
            unicode: unicodeString,
            class: `fas fa-${key}`
        };
    });