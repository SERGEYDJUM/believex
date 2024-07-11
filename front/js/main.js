document.addEventListener("DOMContentLoaded", () => {
    const forecastButton = document.getElementById("get-forecast-btn");

    forecastButton.addEventListener("click", async () => {
        const age = document.getElementById("age").value;
        const gender = document.getElementById("gender").value;
        const vlf = document.getElementById("vlf").value;
        const lf = document.getElementById("lf").value;
        const hf = document.getElementById("hf").value;

        try {
            const response = await fetch(`http://localhost:7331/infer?lf=${lf}&hf=${hf}&sex=${gender}&age=${age}`);
            if (!response.ok) {
                throw new Error("Network response was not ok");
            }
            const data = await response.json();

            // Предполагаем, что API возвращает значения в формате { hlf_2h: ..., lf_2h: ..., hf_2h: ..., hlf_5d: ..., lf_5d: ..., hf_5d: ... }
            document.getElementById("hlf-1").textContent = data.hlf_2h;
            document.getElementById("lf-1").textContent = data.lf_2h;
            document.getElementById("hf-1").textContent = data.hf_2h;

            document.getElementById("hlf-2").textContent = data.hlf_5d;
            document.getElementById("lf-2").textContent = data.lf_5d;
            document.getElementById("hf-2").textContent = data.hf_5d;
        } catch (error) {
            console.error("Fetch error: ", error);
        }
    });
});
