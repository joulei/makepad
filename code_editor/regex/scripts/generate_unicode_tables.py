#!/usr/bin/python3

"""Generating Unicode tables"""

import argparse
import re
import sys

class Error(Exception):
    """Error base class."""


def parse_code_point(string):
    """Parses a Unicode code point.

    Code points are expressed as hexadecimal numbers with four to six digits.
    """
    if len(string) < 4 or len(string) > 6:
        raise Error("invalid code point %s" % string)
    try:
        code_point = int(string, 16)
    except ValueError:
        raise Error("invalid code point %s" % string)
    if code_point < 0 or code_point > sys.maxunicode:
        raise Error("invalid code point %s" % string)
    return code_point


def parse_code_point_range(string):
    """Parses a range of Unicode code points.

    A range of code points is specified either by the form "X..Y", where X is
    less than or equal to Y, or by the form "X", which is short for "X..X".
    """
    bounds = string.split("..")
    if len(bounds) == 1:
        code_point = parse_code_point(bounds[0])
        return range(code_point, code_point + 1)
    elif len(bounds) == 2:
        first_code_point = parse_code_point(bounds[0])
        last_code_point = parse_code_point(bounds[1])
        if first_code_point > last_code_point:
            raise Error("invalid code point range %s" % string)
        return range(first_code_point, last_code_point + 1)
    else:
        raise Error("invalid code point range %s" % string)


def parse_character_name(string):
    """Parses a Unicode character name.

    For backward compatibility, ranges in the file UnicodeData.txt are
    specified by entries for the start and end characters of the range, rather
    than by the form "X..Y". The start character is indicated by a range
    identifier, followed by a comma and the string "First", in angle brackets.
    This line takes the place of a regular character name in field 1 for that
    line. The end character is indicated on the next line with the same range
    identifier, followed by a comma and the string "Last", in angle brackets.
    """
    match = re.match("<(.*), (First|Last)>", string)
    if match is not None:
        return match.groups()
    return (string, None)


def read_unicode_data(filename, expected_field_count):
    """A reader for Unicode data files.

    The reader strips out comments and whitespace, and skips empty lines. For
    non-empty lines, the reader splits the line into fields, checks if the
    line has the expected number of fields, and strips out whitespace for each
    field.

    The reader also takes care of parsing code point ranges. Unicode data
    files specify these in two different ways, either by the form "X..Y", or
    by entries for the start and end characters of the range.
    """
    file = open(filename, encoding="utf8")
    lineno = 1
    first = None
    expected_name = None
    for line in file:
        # Strip out comments and whitespace, and skip empty lines.
        hash = line.find("#")
        if hash >= 0:
            line = line[:hash]
        line = line.strip()
        if not line:
            continue

        try:
            # Split the line into fields, check if the line has the expected
            # number of fields, and strip out whitespace for each field.
            fields = [field.strip() for field in line.split(";")]
            if len(fields) != expected_field_count:
                raise ValueError("invalid number of fields %d" % len(fields))

            # Unicode data files specify code point ranges in two different
            # ways, either by the form "X..Y", or by entries for the start and
            # end characters of the range. 
            code_points = parse_code_point_range(fields[0])
            (name, first_or_last) = parse_character_name(fields[1])
            if expected_name is None:
                if first_or_last == "First":
                    # The current line is the first entry for a range.
                    # Remember it and continue with the next line.
                    if len(code_points) != 1:
                        raise ValueError("invalid First line")
                    expected_name = name
                    first = code_points[0]
                    continue
            else:
                # If the previous line was the first entry for a range, the
                # current line should be the last entry for that range.
                if name != expected_name or first_or_last != "Last" or len(
                        code_points) != 1 or code_points[0] < first:
                    raise ValueError("invalid Last line")
                code_points = range(first, code_points[0] + 1)
                fields[1] = name
                first = None
                expected_name = None
        except Exception as e:
            e.args = ("%s:%d:" % (filename, lineno), *e.args)
            raise e.with_traceback(e.__traceback__)   
        fields[0] = code_points
        yield fields
        lineno += 1


def compute_delta(a, b):
    if a + 1 == b:
        if a % 2 == 0:
            return 1
        else:
            return -1
    if a == b + 1:
        if a % 2 == 0:
            return -1
        else:
            return 1
    return b - a


def apply_delta(a, delta):
    if delta == 1:
        if a % 2 == 0:
            return a + 1
        else:
            return a - 1
    if delta == -1:
        if a % 2 == 1:
            return a + 1
        else:
            return a - 1
    return a + delta


def generate_case_folding_table(ucd_dir):
    groups = {}
    for [code_points, status, mapping, *_] in read_unicode_data(ucd_dir + "/CaseFolding.txt", 4):
        if status not in ("C", "S"):
            continue
        mapping = parse_code_point(mapping)
        groups.setdefault(mapping, [mapping]).extend(code_points)
    groups = list(groups.values())
    for group in groups:
        group.sort()
    groups.sort()
    pairs = []
    for group in groups:
        for index in range(len(group)):
            pairs.append([group[index - 1], group[index]])
    pairs.sort()
    entries = []
    for a, b in pairs:
        if entries:
            [_, prev_last, prev_delta] = entries[-1]
            if a == prev_last + 1 and b == apply_delta(a, prev_delta):
                entries[-1][1] = a
                continue
        entries.append([a, a, compute_delta(a, b)])
    print("use crate::Range;")
    print()
    print("pub(crate) static CASE_FOLDING: [(Range<char>, i32); %d] = [" % len(entries))
    for [first, last, delta] in entries:
        print("    (Range::new('\\u{%06x}', '\\u{%06x}'), %d)," % (first, last, delta))
    print("];")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("property_name", type=str)
    parser.add_argument("ucd_dir", type=str)
    args = parser.parse_args()
    if args.property_name == "CaseFolding":
        generate_case_folding_table(args.ucd_dir) 

if __name__ == '__main__':
    main()
