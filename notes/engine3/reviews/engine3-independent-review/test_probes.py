"""Standard-library tests of the independent probes and budget reference only."""
import math
import random
import unittest
from probes import (remaining_after_stamps, ideal_path_counts,
                    effective_flow_minutes, scheduled_flow_minutes, film_cap_um)
from bounded_transfer import Request, allocate, move_components

class ScalarProbes(unittest.TestCase):
    def test_zero_contact(self):
        self.assertEqual(remaining_after_stamps(23.6, 0, 88), 1)
    def test_full_contact_exponential(self):
        self.assertAlmostEqual(remaining_after_stamps(3, 1, 88), math.exp(-3), places=13)
    def test_partial_contact_can_clear_almost_all(self):
        self.assertLess(remaining_after_stamps(23.6, .2, 88), .02)
    def test_continuous_limit(self):
        self.assertAlmostEqual(remaining_after_stamps(23.6, .2, 100000), math.exp(-4.72), places=5)
    def test_polyline_partition_changes_mechanics_count(self):
        a = ideal_path_counts([176])
        b = ideal_path_counts([.1]*1760)
        self.assertEqual(a, {"paint_stamps":352, "mechanical_steps":704})
        self.assertEqual(b, {"paint_stamps":1760, "mechanical_steps":1760})
    def test_uncapped_flow_full_time(self):
        self.assertAlmostEqual(effective_flow_minutes(2400, .5)["effective_flow_minutes"], 1)
    def test_capped_flow_resolution_scaling(self):
        a = effective_flow_minutes(2400, .95)["effective_flow_minutes"]
        b = effective_flow_minutes(4800, .95)["effective_flow_minutes"]
        self.assertAlmostEqual(a / b, 4)
        self.assertLess(a, .38)
    def test_absolute_clock_phase(self):
        self.assertEqual(scheduled_flow_minutes(.1, .02), 0)
        self.assertEqual(scheduled_flow_minutes(.99, .02), 1)
    def test_thinner_cap_is_on_liquid(self):
        self.assertEqual(film_cap_um(.5)["liquid_cap_um"], 6)
        self.assertEqual(film_cap_um(.5)["nonvolatile_cap_if_same_composition_um"], 3)
    def test_blot_spatial_absorption_value(self):
        self.assertAlmostEqual(1-.65**3.2, .748045139243092, places=14)

class BudgetReference(unittest.TestCase):
    def test_full_receiver_stays_full(self):
        got = allocate([1], [0, 1], [Request(0,0,.1), Request(0,1,.1)])
        self.assertEqual(got, (Request(0,1,.1),))
    def test_donor_and_receiver_limits(self):
        got = allocate([1, 1], [.5, 10], [Request(0,0,2),Request(0,1,2),Request(1,0,2)])
        self.assertLessEqual(sum(t.volume for t in got if t.donor == 0), 1)
        self.assertAlmostEqual(sum(t.volume for t in got if t.receiver == 0), .5)
    def test_no_contact_no_transfer(self):
        self.assertEqual(allocate([1],[1],[]), ())
    def test_duplicate_edges_and_order(self):
        req = [Request(0,0,.1), Request(0,0,.2), Request(1,0,.5)]
        self.assertEqual(allocate([1,1],[.2],req), allocate([1,1],[.2],req[::-1]))
    def test_each_component_conserved(self):
        d, r = [[.2,.8], [.7,.3]], [[.1,.4],[0,0]]
        transfers = allocate([1,1],[.5,.8],[Request(0,0,1),Request(1,0,1),Request(1,1,1)])
        nd, nr = move_components(d,r,transfers)
        for k in range(2):
            self.assertAlmostEqual(sum(x[k] for x in d+r),sum(x[k] for x in nd+nr),places=14)
        self.assertTrue(all(x >= 0 for row in nd+nr for x in row))
    def test_overdraw_rejected(self):
        with self.assertRaises(ValueError):
            move_components([[1]], [[0]], [Request(0,0,2)])
    def test_invalid_inputs_rejected(self):
        with self.assertRaises(ValueError): allocate([float('nan')],[1],[])
        with self.assertRaises(IndexError): allocate([1],[1],[Request(3,0,1)])
        with self.assertRaises(ValueError): allocate([1],[1],[Request(0,0,-1)])
    def test_random_budget_and_conservation_properties(self):
        rng=random.Random(20261004)
        for _ in range(500):
            av=[rng.random()*3 for _ in range(4)]
            spare=[rng.random()*2 for _ in range(3)]
            requests=[Request(rng.randrange(4),rng.randrange(3),rng.random()*4) for _ in range(30)]
            transfers=allocate(av,spare,requests)
            self.assertEqual(transfers,allocate(av,spare,requests[::-1]))
            for i,a in enumerate(av): self.assertLessEqual(math.fsum(t.volume for t in transfers if t.donor==i),a+1e-12)
            for j,c in enumerate(spare): self.assertLessEqual(math.fsum(t.volume for t in transfers if t.receiver==j),c+1e-12)
            donors=[[.3*a,.7*a] for a in av]
            receivers=[[0,0] for _ in spare]
            nd,nr=move_components(donors,receivers,transfers)
            for k in range(2): self.assertAlmostEqual(math.fsum(row[k] for row in donors),math.fsum(row[k] for row in nd+nr),places=12)

if __name__ == '__main__': unittest.main(verbosity=2)
