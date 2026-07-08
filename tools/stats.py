# pip install matplotlib

import os
import numpy as np
import matplotlib.pyplot as plt

def safe_float(x):
    try:
        return float(x)
    except:
        return np.nan

def safe_pct(num, den):
    if den == 0 or den is None or not np.isfinite(den):
        return np.nan
    return 100.0 * num / den

def ask_float(prompt):
    while True:
        v = safe_float(input(prompt).strip().replace(",", "."))
        if np.isfinite(v):
            return v
        print("Entrada inválida. Reintenta.")

def generate_coverage_scalar_report(bloques_totales, bloques_modificados, bloques_no_modificados,
                                     out_dir="output_coverage_scalar", title="Cobertura de bloques (modificación)"):
    os.makedirs(out_dir, exist_ok=True)

    total = float(bloques_totales)
    mod = float(bloques_modificados)
    not_mod = float(bloques_no_modificados)

    
    bloques_sin_fallo = total - mod - not_mod
    free = float(bloques_sin_fallo) 

    pct_mod = safe_pct(mod, total)
    pct_no_mod = safe_pct(not_mod, total)
    pct_free = safe_pct(free, total)

    # Validación suave (por si te dan números inconsistentes)
    if np.isfinite(total) and total < 0:
        print("Aviso: bloques totales < 0")
    if any(np.isfinite(v) and v < 0 for v in (mod, not_mod, free)):
        print("Aviso: hay valores negativos (revisa la entrada).")

    with open(os.path.join(out_dir, "estadisticas_resumen.txt"), "w", encoding="utf-8") as f:
        f.write(f"{title}\n")
        f.write(f"Bloques totales: {total:.6g}\n")
        f.write(f"Bloques modificados: {mod:.6g}\n")
        f.write(f"Bloques no modificados (no se pudo): {not_mod:.6g}\n")
        f.write(f"Bloques Unsafe genericos: {free:.6g}\n\n")
        f.write("Tasas sobre totales:\n")
        f.write(f"- Modificados: {pct_mod:.2f} %\n" if np.isfinite(pct_mod) else "- Modificados: NaN\n")
        f.write(f"- No modificados: {pct_no_mod:.2f} %\n" if np.isfinite(pct_no_mod) else "- No modificados: NaN\n")
        f.write(f"-Unsafe genericos: {pct_free:.2f} %\n" if np.isfinite(pct_free) else "Unsafe genericos: NaN\n")

    labels = ["Modificados", "No modificados (no se pudo)", "Unsafe generic"]
    values = [mod, not_mod, free]
    colors = ["#2ca02c", "#d62728", "#1f77b4"]

    # 100% apilada
    pct = [safe_pct(v, total) if total > 0 else 0 for v in values]
    fig, ax = plt.subplots(figsize=(9, 3.8))
    left = 0
    for i, p in enumerate(pct):
        p = 0 if not np.isfinite(p) else p
        ax.barh([0], [p], left=left, color=colors[i], alpha=0.9, height=0.55, label=labels[i])
        if total > 0 and values[i] > 0 and p > 0:
            ax.text(left + p/2, 0, f"{p:.1f}%", ha="center", va="center",
                    fontsize=10, color="white")
        left += p

    ax.set_xlim(0, 100 if total > 0 else 1)
    ax.set_yticks([0])
    ax.set_yticklabels(["Cobertura"])
    ax.set_xlabel("Porcentaje sobre bloques totales")
    ax.set_title("Cobertura por resultado")
    ax.legend(loc="lower right", frameon=True)
    fig.tight_layout()
    fig.savefig(os.path.join(out_dir, "barra_apilada_100pct.png"), dpi=200)
    plt.close(fig)

    # torta
    fig, ax = plt.subplots(figsize=(6.2, 6.2))
    if total > 0:
        pie_vals = [max(v, 0) for v in values]
        ax.pie(pie_vals, labels=labels, autopct="%.1f%%", startangle=90, colors=colors)
        ax.set_title("Cobertura (torta)")
    else:
        ax.text(0.5, 0.5, "Total = 0\nNo hay cobertura para graficar", ha="center", va="center")
        ax.set_axis_off()
    fig.tight_layout()
    fig.savefig(os.path.join(out_dir, "torta_cobertura.png"), dpi=200)
    plt.close(fig)

    # barras conteos
    fig, ax = plt.subplots(figsize=(9, 4))
    cat = ["Totales", "Modificados", "No modificados", "Unsafe genericos"]
    vals = [total, mod, not_mod, free]
    ax.bar(cat, vals, color=["#444444", "#2ca02c", "#d62728", "#1f77b4"], alpha=0.9)
    ax.set_ylabel("Cantidad de bloques")
    ax.set_title("Cobertura (conteos)")
    fig.tight_layout()
    fig.savefig(os.path.join(out_dir, "barras_conteos.png"), dpi=200)
    plt.close(fig)

    print(f"Listo. Archivos generados en: {os.path.abspath(out_dir)}")
    print("Incluye: estadisticas_resumen.txt, barra_apilada_100pct.png, torta_cobertura.png, barras_conteos.png")

if __name__ == "__main__":
    print("=== Cálculo de cobertura de bloques ===")
    bloques_totales = ask_float("Bloques totales (cantidad): ")
    bloques_modificados = ask_float("Bloques modificados: ")
    bloques_no_modificados = ask_float("Bloques no modificados (no se pudo): ")

    generate_coverage_scalar_report(bloques_totales, bloques_modificados, bloques_no_modificados)
