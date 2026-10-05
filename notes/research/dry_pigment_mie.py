import numpy as np
def bhmie(x, m):
    # Bohren & Huffman, returns qext, qsca, qabs, g
    nstop = int(x + 4*x**(1/3) + 2)
    y = m*x
    nmx = int(max(nstop, abs(y)) + 15)
    D = np.zeros(nmx+1, dtype=complex)
    for n in range(nmx, 0, -1):
        D[n-1] = n/y - 1/(D[n] + n/y)
    psi0, psi1 = np.cos(x), np.sin(x)
    chi0, chi1 = -np.sin(x), np.cos(x)
    xi1 = complex(psi1, -chi1)
    qsca = 0; qext = 0; g = 0
    an1 = bn1 = None
    for n in range(1, nstop+1):
        psi = (2*n-1)*psi1/x - psi0
        chi = (2*n-1)*chi1/x - chi0
        xi = complex(psi, -chi)
        an = ((D[n]/m + n/x)*psi - psi1)/((D[n]/m + n/x)*xi - xi1)
        bn = ((D[n]*m + n/x)*psi - psi1)/((D[n]*m + n/x)*xi - xi1)
        qsca += (2*n+1)*(abs(an)**2 + abs(bn)**2)
        qext += (2*n+1)*(an.real + bn.real)
        if n > 1:
            g += (n-1)*(n+1)/n*(an1*an.conjugate() + bn1*bn.conjugate()).real + (2*n-1)/((n-1)*n)*(an1*bn1.conjugate()).real
        an1, bn1 = an, bn
        psi0, psi1 = psi1, psi
        chi0, chi1 = chi1, chi
        xi1 = complex(psi1, -chi1)
    qsca *= 2/x**2; qext *= 2/x**2
    g *= 4/(x**2*qsca)
    return qext, qsca, qext-qsca, g

def sizeavg(npart, nmed, d_med, lam=0.55, sig=0.5, N=41):
    # lognormal number dist (geometric std exp(sig)) of diameters, volume-normalised
    ds = d_med*np.exp(np.linspace(-3*sig, 3*sig, N))
    w = np.exp(-0.5*(np.log(ds/d_med)/sig)**2)
    S=A=V=0
    for d,wi in zip(ds,w):
        x = np.pi*d*nmed/lam
        qe,qs,qa,g = bhmie(x, npart/nmed)
        area = np.pi*d*d/4
        S += wi*area*qs*(1-g); A += wi*area*qa; V += wi*np.pi*d**3/6
    return S/V, A/V   # transport-scattering and absorption cross-section per unit particle volume (1/um)

if __name__ == "__main__":
    mats = [("lead white",2.0),("zinc white",2.01),("vermilion",3.0),("hematite (red ochre)",3.0),("goethite ochre",2.3),
            ("chrome yellow",2.4),("cadmium yellow",2.45),("Naples yellow",2.15),("red lead",2.42),
            ("cobalt blue",1.74),("cerulean",1.84),("emerald green",1.75),("viridian",1.8),("cobalt violet",1.7),
            ("Prussian blue",1.56),("ultramarine",1.50),("alumina lake",1.55),("calcite chalk",1.59),("kaolin",1.56),("gypsum",1.525),("bone black matrix",1.65)]
    print("%-22s %5s | " % ("material","n") + " ".join("d=%-4s air/oil1.48 air/oil1.57" % d for d in ("0.3","1","3")))
    for name,n in mats:
        row=[]
        for d in (0.3,1.0,3.0):
            sa,_ = sizeavg(n,1.0,d)
            so,_ = sizeavg(n,1.48,d)
            s2,_ = sizeavg(n,1.57,d)
            row.append("%6.2f %8s %8s" % (sa, "%.1f"%(sa/so) if so>1e-6 else "inf", "%.1f"%(sa/s2) if s2>1e-6 else "inf"))
        print("%-22s %5.2f | " % (name,n) + " | ".join(row))
