import re, subprocess, rich, sys, os, datetime
import pandas as pd

edhoc_rs_compilation_units_pattern = "edhoc_rs_no_std|liblakers|libedhoc_crypto"
other_compilation_units_pattern = "libcortex_m_rt|libcompiler_builtins|libcore|libcortex_m_rt|libcortex_m|libcortex_m_semihosting|libpanic_semihosting|librtt_target"

def run_cmd(cmd):
    print_debug(f"Will run: {cmd}")
    res = subprocess.run(cmd, shell=True, capture_output=True, text=True)
    if res.returncode != 0:
        raise Exception(f"Failed to run: {cmd}")
    print_debug(f"Run ok.")
    return res.stdout

def print_debug(*args):
    # return
    rich.print(*args)

def get_between(marker_start, marker_end, map_file):
    return run_cmd(f"gsed -n '/{marker_start}/,/{marker_end}/{{ /{marker_end}/!p }}' {map_file}")

def run(map_file):
    region = get_between(".text$", ".rodata$", map_file)
    symbols = []
    for line in region.split("\n"):
        # print(line)
        hex_num_r = "([0-9a-f]+)"
        parsed_line = re.match(f"^\s+{hex_num_r}\s+{hex_num_r}\s+{hex_num_r}\s+{hex_num_r}\s+(\/.*\))$", line)
        if parsed_line:
            # print_debug(f"+++++++: {line}")
            _, _, size, align, name = parsed_line.group(1, 2, 3, 4, 5)
            size = int(size, 16)
            # parsed_name = re.match(f"^(.*)", name)

            stype = ""
            if re.match(f".*\.o:\(.*", name):
                stype = ".o"
            if re.match(f".*\.rlib.*", name):
                stype = ".rlib"
            symbols.append((size, stype, name))

            # if parsed_name := re.match(f".*({edhoc_rs_compilation_units_pattern})", name):
            #     print(parsed_name.group(1))
        else:
            # print_debug(f"skipped: {line}")
            pass
    symbols = sorted(symbols, key=lambda x: (-x[0], stype))
    rich.print(symbols[:20])
    # print(sum([s for s, v in symbols]))

run(sys.argv[1])
