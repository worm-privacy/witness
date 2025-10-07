import io
import os

os.system(
    "wget -c https://github.com/worm-privacy/trusted-setup/releases/download/circuit_data/proof_of_burn_witness_gen.tar.gz"
)
os.system(
    "wget -c https://github.com/worm-privacy/trusted-setup/releases/download/circuit_data/spend_witness_gen.tar.gz"
)
os.system(
    "rm -rf proof_of_burn_witness_gen && mkdir proof_of_burn_witness_gen && tar xzf proof_of_burn_witness_gen.tar.gz -C proof_of_burn_witness_gen --strip-components 4"
)
os.system(
    "rm -rf spend_witness_gen && mkdir spend_witness_gen && tar xzf spend_witness_gen.tar.gz -C spend_witness_gen --strip-components 4"
)

for circuit in ["spend", "proof_of_burn"]:
    with io.open(f"{circuit}_witness_gen/calcwit.cpp", "r+") as f:
        s = f.read()
        inc_ind = s.rindex("#include")
        final_ind = inc_ind + s[inc_ind:].index("\n")
        s = (
            s[:final_ind]
            + f"\nnamespace {circuit} {{ // NAMESPACE BEGIN\n"
            + s[final_ind:]
            + "\n} // NAMESPACE END"
        )
        f.seek(0)
        f.write(s)
        f.truncate()

    with io.open(f"{circuit}_witness_gen/verify_circuit.cpp", "r+") as f:
        s = f.read()
        inc_ind = s.rindex("#include")
        final_ind = inc_ind + s[inc_ind:].index("\n")
        s = (
            s[:final_ind]
            + f"\nnamespace {circuit} {{ // NAMESPACE BEGIN\n"
            + s[final_ind:]
            + "\n} // NAMESPACE END"
        )
        f.seek(0)
        f.write(s)
        f.truncate()

    with io.open(f"{circuit}_witness_gen/calcwit.hpp", "r+") as f:
        s = f.read()
        s = s.replace('#include "fr.hpp"', '#include "../fr/fr.hpp"')
        inc_ind = s.rindex("#include")
        final_ind = inc_ind + s[inc_ind:].index("\n")
        endif_ind = s.rindex("#endif")
        s = (
            s[:final_ind]
            + f"\nnamespace {circuit} {{ // NAMESPACE BEGIN\n"
            + s[final_ind:endif_ind]
            + "\n} // NAMESPACE END\n"
            + s[endif_ind:]
        )
        f.seek(0)
        f.write(s)
        f.truncate()

    with io.open(f"{circuit}_witness_gen/circom.hpp", "r+") as f:
        s = f.read()
        s = s.replace('#include "fr.hpp"', '#include "../fr/fr.hpp"')
        inc_ind = s.rindex("#include")
        final_ind = inc_ind + s[inc_ind:].index("\n")
        endif_ind = s.rindex("#endif")
        s = (
            s[:final_ind]
            + f"\nnamespace {circuit} {{ // NAMESPACE BEGIN\n"
            + s[final_ind:endif_ind]
            + "\n} // NAMESPACE END\n"
            + s[endif_ind:]
        )
        f.seek(0)
        f.write(s)
        f.truncate()

    with io.open(f"{circuit}_witness_gen/main.cpp", "r+") as f:
        s = f.read()
        main_start = s.rindex("int main")
        s = s[:main_start]
        inc_ind = s.rindex("#include")
        final_ind = inc_ind + s[inc_ind:].index("\n")
        s = (
            (
                s[:final_ind]
                + f"\nnamespace {circuit} {{ // NAMESPACE BEGIN\n"
                + s[final_ind:]
                + "\n} // NAMESPACE END\n"
            )
            + f"""
        extern "C"
        {{
        using namespace {circuit};
        int gen_{circuit}_witness_file(char const *datfile, char const *jsonfile, char *wtnsfile, char *errmsg)
        {{
            try
            {{
            Circom_Circuit *circuit = loadCircuit(std::string(datfile));
        Circom_CalcWit *ctx = new Circom_CalcWit(circuit);
        
            loadJson(ctx, std::string(jsonfile));
            if (ctx->getRemaingInputsToBeSet() != 0)
            {{
                std::ostringstream errStrStream;
                errStrStream << "Not all inputs have been set. Only " << get_main_input_signal_no() - ctx->getRemaingInputsToBeSet() << " out of " << get_main_input_signal_no() << std::endl;
                throw std::runtime_error(errStrStream.str());
            }}

            writeBinWitness(ctx, std::string(wtnsfile));
            return 0;
            
            }}
            catch (std::runtime_error e)
            {{
            strcpy(errmsg, e.what());
            return -1;
            }}
        }}
        }}"""
        )
        f.seek(0)
        f.write(s)
        f.truncate()

    os.system(f"mv {circuit}_witness_gen/calcwit.cpp {circuit}/calcwit.cpp")
    os.system(f"mv {circuit}_witness_gen/calcwit.hpp {circuit}/calcwit.hpp")
    os.system(f"mv {circuit}_witness_gen/circom.hpp {circuit}/circom.hpp")
    os.system(f"mv {circuit}_witness_gen/main.cpp {circuit}/main.cpp")
    os.system(f"mv {circuit}_witness_gen/verify_circuit.cpp {circuit}/{circuit}.cpp")
