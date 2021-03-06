// Parallel DLEQ proofs
use crypto::*;
use dleq;

type Challenge = Scalar;

#[derive(Clone)]
pub struct Proof {
    c: Challenge,
    zs: Vec<Scalar>,
}

impl Proof {
    pub fn create(params: &[(Scalar, Scalar, dleq::DLEQ)]) -> Proof {
        let mut his = Vec::with_capacity(params.len() * 4);
        let mut ais = Vec::with_capacity(params.len() * 2);
        let mut zs = Vec::with_capacity(params.len());

        // create the list [h1_1 ,h2_1 , h1_2 , h2_2, ... h2_n, a1_1, a2_1, .., a1_n, a2_n ]
        // to compute the challenge
        for param in params.iter() {
            let &(ref w, _, ref dleq) = param;
            his.push(dleq.h1.clone());
            his.push(dleq.h2.clone());
            ais.push(dleq.g1.mul(&w));
            ais.push(dleq.g2.mul(&w));
        }

        // compute the challenge
        his.append(&mut ais);
        let c = Scalar::hash_points(his);

        // finally create each proofs
        for param in params.iter() {
            let &(ref w, ref a, _) = param;
            let z = w.clone() + a.clone() * c.clone();
            zs.push(z);
        }
        return Proof { c: c, zs: zs };
    }

    pub fn verify(&self, dleqs: &[dleq::DLEQ]) -> bool {
        let mut his = Vec::new();
        let mut ais = Vec::new();

        if dleqs.len() != self.zs.len() {
            // FIXME probably an Err() .. instead of silent verify failure
            return false;
        };

        // recompute the challenge
        for i in 0..self.zs.len() {
            let z = &self.zs[i];
            let dleq = &dleqs[i];
            let r1 = dleq.g1.mul(z);
            let r2 = dleq.g2.mul(z);
            let a1 = r1 - dleq.h1.mul(&self.c);
            let a2 = r2 - dleq.h2.mul(&self.c);
            his.push(dleq.h1.clone());
            his.push(dleq.h2.clone());
            ais.push(a1);
            ais.push(a2);
        }

        his.append(&mut ais);
        let c = Scalar::hash_points(his);

        return self.c == c;
    }
}
