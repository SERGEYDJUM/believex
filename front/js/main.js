async function forecast() {
    const gender = document.getElementById("gender").value;
    const age = document.getElementById("age").value;
    const lf = document.getElementById("lf").value;
    const hf = document.getElementById("hf").value;

    const response_2h = await fetch(`/infer?lf_b=${lf}&hf_b=${hf}&sex=${gender}&age=${age}&otime=After2h`);
    const response_5d = await fetch(`/infer?lf_b=${lf}&hf_b=${hf}&sex=${gender}&age=${age}&otime=After5d`);

    if (!response_2h.ok || !response_5d.ok) {
        throw new Error("Проверьте введённые данные");
    }

    const data_2h = await response_2h.json();
    const data_5d = await response_5d.json();

    document.getElementById("lf-1").textContent = data_2h.lf;
    document.getElementById("hf-1").textContent = data_2h.hf;

    document.getElementById("lf-2").textContent = data_5d.lf;
    document.getElementById("hf-2").textContent = data_5d.hf;
}

document.addEventListener("DOMContentLoaded", () => {
    document.getElementById("get-forecast-btn").onclick = () => forecast().catch((err) => alert(err));
});
