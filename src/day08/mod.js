#!/usr/bin/env node

const fs = require ('fs');
const assert = require ('assert');

const count_intersects = (a, b) => {
    return String (b).split ('').reduce ((acc, elem) => {
        return acc + (String (a).split ('').includes (elem) ? 1 : 0);
    }, 0)
}

const decode_inputs = (inputs) => {
    let decoded = new Map();

    const set_decoded_digit = (digit, input) => {
        decoded.set (digit, input);
        inputs.delete (input);
    }

    const count_intersects_with_digit = (n) => {
        return (input) => count_intersects (input, decoded.get (n));
    }

    const find_input = (predicate) => {
        const match = Array.from (inputs).filter (predicate);
        assert.equal (match.length, 1); // all being well we should have exactly one match...
        return match[0];
    };

    set_decoded_digit (1, find_input (input => input.length === 2));
    set_decoded_digit (7, find_input (input => input.length === 3));
    set_decoded_digit (4, find_input (input => input.length === 4));
    set_decoded_digit (8, find_input (input => input.length === 7));

    set_decoded_digit (2, find_input (input => input.length === 5 && count_intersects_with_digit (4) (input) === 2));
    set_decoded_digit (5, find_input (input => input.length === 5 && count_intersects_with_digit (1) (input) === 1));
    set_decoded_digit (3, find_input (input => input.length === 5 && count_intersects_with_digit (1) (input) === 2));

    set_decoded_digit (9, find_input (input => input.length === 6 && count_intersects_with_digit (3) (input) === 5));
    set_decoded_digit (0, find_input (input => input.length === 6 && count_intersects_with_digit (1) (input) === 2));
    set_decoded_digit (6, find_input (input => input.length === 6 && count_intersects_with_digit (1) (input) === 1));

    return new Map (Array.from (decoded, a => a.reverse()));
}

const input_lines = () => fs.readFileSync ('input.txt').toString ('utf8').split ('\n');

const sort_string = (string) => string.split ('').sort().join ('');

const part_one = () => {
    return input_lines().reduce ((appearances, line) => {
        const [_, output] = line.split (' | ');
        return appearances + output.split  (' ')
            .map (digit => digit.length)
            .filter (length => [2, 3, 4, 7].includes (length))
            .length;
    }, 0);
};

const part_two = () => {
    return input_lines().reduce ((sum, line) => {
        const [input, output] = line.split (' | ');

        const decoded = decode_inputs (new Set (input.split (' ').map (sort_string)));

        const decode_key = (key) => decoded.get (key);
        let decoded_output = output.split (' ')
            .map (sort_string)
            .map (decode_key)
            .join ('');

        return sum + parseInt (decoded_output);
    }, 0);
}

assert.equal (part_one(), 452);
assert.equal (part_two(), 1096964);