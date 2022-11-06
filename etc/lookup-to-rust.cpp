// The following license regards to the lookup table used below.
// The original source code can be found on
// https://github.com/OneLoneCoder/olcNES
//
// License (OLC-3)
// ~~~~~~~~~~~~~~~
//
// Copyright 2018-2019 OneLoneCoder.com
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
// 1. Redistributions or derivations of source code must retain the above
// copyright notice, this list of conditions and the following disclaimer.
//
// 2. Redistributions or derivative works in binary form must reproduce
// the above copyright notice. This list of conditions and the following
// disclaimer must be reproduced in the documentation and/or other
// materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its
// contributors may be used to endorse or promote products derived
// from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#include <string>
#include <vector>
#include <algorithm>
#include <cctype>

std::string lower(std::string str) {
    std::string data = std::string(str);
    std::transform(data.begin(), data.end(), data.begin(),
        [](unsigned char c){ return std::tolower(c); });
    return data;
}

struct Instruction {
    std::string name;
    std::string operate;
    std::string addrmode;
    uint8_t cycles = 0;
};

std::vector<Instruction> lookup = 
{
    { "BRK", "BRK", "IMM", 7 },{ "ORA", "ORA", "IZX", 6 },{ "???", "XXX", "IMP", 2 },{ "???", "XXX", "IMP", 8 },{ "???", "NOP", "IMP", 3 },{ "ORA", "ORA", "ZP0", 3 },{ "ASL", "ASL", "ZP0", 5 },{ "???", "XXX", "IMP", 5 },{ "PHP", "PHP", "IMP", 3 },{ "ORA", "ORA", "IMM", 2 },{ "ASL", "ASL", "IMP", 2 },{ "???", "XXX", "IMP", 2 },{ "???", "NOP", "IMP", 4 },{ "ORA", "ORA", "ABS", 4 },{ "ASL", "ASL", "ABS", 6 },{ "???", "XXX", "IMP", 6 },
    { "BPL", "BPL", "REL", 2 },{ "ORA", "ORA", "IZY", 5 },{ "???", "XXX", "IMP", 2 },{ "???", "XXX", "IMP", 8 },{ "???", "NOP", "IMP", 4 },{ "ORA", "ORA", "ZPX", 4 },{ "ASL", "ASL", "ZPX", 6 },{ "???", "XXX", "IMP", 6 },{ "CLC", "CLC", "IMP", 2 },{ "ORA", "ORA", "ABY", 4 },{ "???", "NOP", "IMP", 2 },{ "???", "XXX", "IMP", 7 },{ "???", "NOP", "IMP", 4 },{ "ORA", "ORA", "ABX", 4 },{ "ASL", "ASL", "ABX", 7 },{ "???", "XXX", "IMP", 7 },
    { "JSR", "JSR", "ABS", 6 },{ "AND", "AND", "IZX", 6 },{ "???", "XXX", "IMP", 2 },{ "???", "XXX", "IMP", 8 },{ "BIT", "BIT", "ZP0", 3 },{ "AND", "AND", "ZP0", 3 },{ "ROL", "ROL", "ZP0", 5 },{ "???", "XXX", "IMP", 5 },{ "PLP", "PLP", "IMP", 4 },{ "AND", "AND", "IMM", 2 },{ "ROL", "ROL", "IMP", 2 },{ "???", "XXX", "IMP", 2 },{ "BIT", "BIT", "ABS", 4 },{ "AND", "AND", "ABS", 4 },{ "ROL", "ROL", "ABS", 6 },{ "???", "XXX", "IMP", 6 },
    { "BMI", "BMI", "REL", 2 },{ "AND", "AND", "IZY", 5 },{ "???", "XXX", "IMP", 2 },{ "???", "XXX", "IMP", 8 },{ "???", "NOP", "IMP", 4 },{ "AND", "AND", "ZPX", 4 },{ "ROL", "ROL", "ZPX", 6 },{ "???", "XXX", "IMP", 6 },{ "SEC", "SEC", "IMP", 2 },{ "AND", "AND", "ABY", 4 },{ "???", "NOP", "IMP", 2 },{ "???", "XXX", "IMP", 7 },{ "???", "NOP", "IMP", 4 },{ "AND", "AND", "ABX", 4 },{ "ROL", "ROL", "ABX", 7 },{ "???", "XXX", "IMP", 7 },
    { "RTI", "RTI", "IMP", 6 },{ "EOR", "EOR", "IZX", 6 },{ "???", "XXX", "IMP", 2 },{ "???", "XXX", "IMP", 8 },{ "???", "NOP", "IMP", 3 },{ "EOR", "EOR", "ZP0", 3 },{ "LSR", "LSR", "ZP0", 5 },{ "???", "XXX", "IMP", 5 },{ "PHA", "PHA", "IMP", 3 },{ "EOR", "EOR", "IMM", 2 },{ "LSR", "LSR", "IMP", 2 },{ "???", "XXX", "IMP", 2 },{ "JMP", "JMP", "ABS", 3 },{ "EOR", "EOR", "ABS", 4 },{ "LSR", "LSR", "ABS", 6 },{ "???", "XXX", "IMP", 6 },
    { "BVC", "BVC", "REL", 2 },{ "EOR", "EOR", "IZY", 5 },{ "???", "XXX", "IMP", 2 },{ "???", "XXX", "IMP", 8 },{ "???", "NOP", "IMP", 4 },{ "EOR", "EOR", "ZPX", 4 },{ "LSR", "LSR", "ZPX", 6 },{ "???", "XXX", "IMP", 6 },{ "CLI", "CLI", "IMP", 2 },{ "EOR", "EOR", "ABY", 4 },{ "???", "NOP", "IMP", 2 },{ "???", "XXX", "IMP", 7 },{ "???", "NOP", "IMP", 4 },{ "EOR", "EOR", "ABX", 4 },{ "LSR", "LSR", "ABX", 7 },{ "???", "XXX", "IMP", 7 },
    { "RTS", "RTS", "IMP", 6 },{ "ADC", "ADC", "IZX", 6 },{ "???", "XXX", "IMP", 2 },{ "???", "XXX", "IMP", 8 },{ "???", "NOP", "IMP", 3 },{ "ADC", "ADC", "ZP0", 3 },{ "ROR", "ROR", "ZP0", 5 },{ "???", "XXX", "IMP", 5 },{ "PLA", "PLA", "IMP", 4 },{ "ADC", "ADC", "IMM", 2 },{ "ROR", "ROR", "IMP", 2 },{ "???", "XXX", "IMP", 2 },{ "JMP", "JMP", "IND", 5 },{ "ADC", "ADC", "ABS", 4 },{ "ROR", "ROR", "ABS", 6 },{ "???", "XXX", "IMP", 6 },
    { "BVS", "BVS", "REL", 2 },{ "ADC", "ADC", "IZY", 5 },{ "???", "XXX", "IMP", 2 },{ "???", "XXX", "IMP", 8 },{ "???", "NOP", "IMP", 4 },{ "ADC", "ADC", "ZPX", 4 },{ "ROR", "ROR", "ZPX", 6 },{ "???", "XXX", "IMP", 6 },{ "SEI", "SEI", "IMP", 2 },{ "ADC", "ADC", "ABY", 4 },{ "???", "NOP", "IMP", 2 },{ "???", "XXX", "IMP", 7 },{ "???", "NOP", "IMP", 4 },{ "ADC", "ADC", "ABX", 4 },{ "ROR", "ROR", "ABX", 7 },{ "???", "XXX", "IMP", 7 },
    { "???", "NOP", "IMP", 2 },{ "STA", "STA", "IZX", 6 },{ "???", "NOP", "IMP", 2 },{ "???", "XXX", "IMP", 6 },{ "STY", "STY", "ZP0", 3 },{ "STA", "STA", "ZP0", 3 },{ "STX", "STX", "ZP0", 3 },{ "???", "XXX", "IMP", 3 },{ "DEY", "DEY", "IMP", 2 },{ "???", "NOP", "IMP", 2 },{ "TXA", "TXA", "IMP", 2 },{ "???", "XXX", "IMP", 2 },{ "STY", "STY", "ABS", 4 },{ "STA", "STA", "ABS", 4 },{ "STX", "STX", "ABS", 4 },{ "???", "XXX", "IMP", 4 },
    { "BCC", "BCC", "REL", 2 },{ "STA", "STA", "IZY", 6 },{ "???", "XXX", "IMP", 2 },{ "???", "XXX", "IMP", 6 },{ "STY", "STY", "ZPX", 4 },{ "STA", "STA", "ZPX", 4 },{ "STX", "STX", "ZPY", 4 },{ "???", "XXX", "IMP", 4 },{ "TYA", "TYA", "IMP", 2 },{ "STA", "STA", "ABY", 5 },{ "TXS", "TXS", "IMP", 2 },{ "???", "XXX", "IMP", 5 },{ "???", "NOP", "IMP", 5 },{ "STA", "STA", "ABX", 5 },{ "???", "XXX", "IMP", 5 },{ "???", "XXX", "IMP", 5 },
    { "LDY", "LDY", "IMM", 2 },{ "LDA", "LDA", "IZX", 6 },{ "LDX", "LDX", "IMM", 2 },{ "???", "XXX", "IMP", 6 },{ "LDY", "LDY", "ZP0", 3 },{ "LDA", "LDA", "ZP0", 3 },{ "LDX", "LDX", "ZP0", 3 },{ "???", "XXX", "IMP", 3 },{ "TAY", "TAY", "IMP", 2 },{ "LDA", "LDA", "IMM", 2 },{ "TAX", "TAX", "IMP", 2 },{ "???", "XXX", "IMP", 2 },{ "LDY", "LDY", "ABS", 4 },{ "LDA", "LDA", "ABS", 4 },{ "LDX", "LDX", "ABS", 4 },{ "???", "XXX", "IMP", 4 },
    { "BCS", "BCS", "REL", 2 },{ "LDA", "LDA", "IZY", 5 },{ "???", "XXX", "IMP", 2 },{ "???", "XXX", "IMP", 5 },{ "LDY", "LDY", "ZPX", 4 },{ "LDA", "LDA", "ZPX", 4 },{ "LDX", "LDX", "ZPY", 4 },{ "???", "XXX", "IMP", 4 },{ "CLV", "CLV", "IMP", 2 },{ "LDA", "LDA", "ABY", 4 },{ "TSX", "TSX", "IMP", 2 },{ "???", "XXX", "IMP", 4 },{ "LDY", "LDY", "ABX", 4 },{ "LDA", "LDA", "ABX", 4 },{ "LDX", "LDX", "ABY", 4 },{ "???", "XXX", "IMP", 4 },
    { "CPY", "CPY", "IMM", 2 },{ "CMP", "CMP", "IZX", 6 },{ "???", "NOP", "IMP", 2 },{ "???", "XXX", "IMP", 8 },{ "CPY", "CPY", "ZP0", 3 },{ "CMP", "CMP", "ZP0", 3 },{ "DEC", "DEC", "ZP0", 5 },{ "???", "XXX", "IMP", 5 },{ "INY", "INY", "IMP", 2 },{ "CMP", "CMP", "IMM", 2 },{ "DEX", "DEX", "IMP", 2 },{ "???", "XXX", "IMP", 2 },{ "CPY", "CPY", "ABS", 4 },{ "CMP", "CMP", "ABS", 4 },{ "DEC", "DEC", "ABS", 6 },{ "???", "XXX", "IMP", 6 },
    { "BNE", "BNE", "REL", 2 },{ "CMP", "CMP", "IZY", 5 },{ "???", "XXX", "IMP", 2 },{ "???", "XXX", "IMP", 8 },{ "???", "NOP", "IMP", 4 },{ "CMP", "CMP", "ZPX", 4 },{ "DEC", "DEC", "ZPX", 6 },{ "???", "XXX", "IMP", 6 },{ "CLD", "CLD", "IMP", 2 },{ "CMP", "CMP", "ABY", 4 },{ "NOP", "NOP", "IMP", 2 },{ "???", "XXX", "IMP", 7 },{ "???", "NOP", "IMP", 4 },{ "CMP", "CMP", "ABX", 4 },{ "DEC", "DEC", "ABX", 7 },{ "???", "XXX", "IMP", 7 },
    { "CPX", "CPX", "IMM", 2 },{ "SBC", "SBC", "IZX", 6 },{ "???", "NOP", "IMP", 2 },{ "???", "XXX", "IMP", 8 },{ "CPX", "CPX", "ZP0", 3 },{ "SBC", "SBC", "ZP0", 3 },{ "INC", "INC", "ZP0", 5 },{ "???", "XXX", "IMP", 5 },{ "INX", "INX", "IMP", 2 },{ "SBC", "SBC", "IMM", 2 },{ "NOP", "NOP", "IMP", 2 },{ "???", "SBC", "IMP", 2 },{ "CPX", "CPX", "ABS", 4 },{ "SBC", "SBC", "ABS", 4 },{ "INC", "INC", "ABS", 6 },{ "???", "XXX", "IMP", 6 },
    { "BEQ", "BEQ", "REL", 2 },{ "SBC", "SBC", "IZY", 5 },{ "???", "XXX", "IMP", 2 },{ "???", "XXX", "IMP", 8 },{ "???", "NOP", "IMP", 4 },{ "SBC", "SBC", "ZPX", 4 },{ "INC", "INC", "ZPX", 6 },{ "???", "XXX", "IMP", 6 },{ "SED", "SED", "IMP", 2 },{ "SBC", "SBC", "ABY", 4 },{ "NOP", "NOP", "IMP", 2 },{ "???", "XXX", "IMP", 7 },{ "???", "NOP", "IMP", 4 },{ "SBC", "SBC", "ABX", 4 },{ "INC", "INC", "ABX", 7 },{ "???", "XXX", "IMP", 7 },
};

enum Mode {
    V1,
    V2
};

int main(int argc, char *argv[]) {
    Mode mode = V2;
    if (argc > 1 && std::string(argv[1]) == "v1") {
        mode = V1;
    }

    switch (mode) {
        case V1:
            for (int opcode = 0; opcode < lookup.size(); opcode++) {
                Instruction ins = lookup[opcode];

                if (ins.name == "???") continue;

                printf("pub const %s_%02X: Instruction = Instruction {\n", ins.name.c_str(), opcode);
                printf("    name: \"%s\",\n", ins.name.c_str());
                printf("    opcode: 0x%02X,\n", opcode);
                printf("    cycles: %d,\n", ins.cycles);
                printf("    addrmode: Cpu::%s,\n", lower(ins.addrmode).c_str());
                printf("    execute: Cpu::%s,\n", lower(ins.operate).c_str());
                printf("};\n");
                printf("\n");
            }
            break;

        case V2:
            printf("build_definitions![\n");

            for (int opcode = 0; opcode < lookup.size(); opcode++) {
                Instruction ins = lookup[opcode];

                if (ins.name == "???") {
                    ins.name = "XXX";
                }

                if (ins.operate == "XXX") {
                    ins.operate = "NOP";
                }

                printf("    (X%02X_%s, 0x%02X, %d, Cpu::%s, Cpu::%s),\n",
                    opcode,
                    ins.name.c_str(),
                    opcode,
                    ins.cycles,
                    lower(ins.addrmode).c_str(),
                    lower(ins.operate).c_str()
                );
            }

            printf("];\n");
            break;
    }

    return 0;
}
