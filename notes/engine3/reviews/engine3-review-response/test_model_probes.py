"""Tests of the isolated calculations only; none executes or validates Rust."""
import unittest
import model_probes as p

class ModelProbeTests(unittest.TestCase):
    def test_source_stain_floor_is_one_micrometre(self):
        self.assertEqual(p.minimum_rag_floor_um(), 1.0)

    def test_submicron_paint_cannot_be_lifted_under_floor_rule(self):
        for h in (0.0, 0.0666666667, 0.5, 0.99, 1.0):
            self.assertEqual(p.most_liftable_um(h), 0.0)
        self.assertAlmostEqual(p.most_liftable_um(1.2), 0.2)

    def test_nominal_rag_threshold(self):
        self.assertAlmostEqual(p.nominal_paint_ceiling_um(2/3), 1.0)
        self.assertGreater(p.nominal_paint_ceiling_um(0.66), 1.0)
        self.assertLess(p.nominal_paint_ceiling_um(0.67), 1.0)

    def test_nominal_flow_threshold(self):
        self.assertEqual(p.stroke_limit_um(0.75), 2.0)
        self.assertGreater(p.flow_available_um(p.stroke_limit_um(0.74)), 0.0)
        self.assertEqual(p.flow_available_um(p.stroke_limit_um(0.76)), 0.0)

    def test_high_thinner_nonvolatile_and_liquid_units_are_distinct(self):
        self.assertAlmostEqual(p.stroke_limit_um(0.9), 2/3)
        self.assertAlmostEqual(p.nominal_paint_ceiling_um(0.9), 1/15)

    def test_unclipped_active_mobility_advances_one_minute(self):
        r = p.flow_step_summary(p.mobility(0.5), 440/2400, 1.0)
        self.assertEqual(r['required_substeps'], 9)
        self.assertAlmostEqual(r['effective_dt_min'], 1.0)

    def test_inactive_high_mobility_can_set_shared_time_coefficient(self):
        self.assertEqual(p.flow_available_um(p.stroke_limit_um(0.95)), 0.0)
        r = p.flow_step_summary(p.mobility(0.95), 440/2400, 1.0)
        self.assertEqual(r['used_substeps'], 64)
        self.assertGreater(r['required_substeps'], 64)
        self.assertAlmostEqual(r['effective_dt_min'], 0.37738791423001984)

    def test_uniform_loading_is_double_throttled_in_example(self):
        single = 1 - 0.8**2
        self.assertAlmostEqual(single, 0.36)
        self.assertAlmostEqual(single**2, 0.1296)

    def test_scalar_hiding_input_is_inverted_by_source_equations(self):
        for r, h in ((0.1735, 0.4), (0.0797, 0.45)):
            self.assertAlmostEqual(p.hiding_of(r, p.scatter_for(r, h)), h, places=9)

    def test_synthetic_one_coat_ratios_match_packet_rounding(self):
        self.assertAlmostEqual(p.sienna_measurement('raw', 25)['actual_rgb_contrast_ratio'], 0.452, delta=0.0005)
        self.assertAlmostEqual(p.sienna_measurement('burnt', 25)['actual_rgb_contrast_ratio'], 0.438, delta=0.0005)

    def test_same_substrates_still_give_opposite_metric_rankings(self):
        for um in (3,10,25,30):
            raw = p.sienna_measurement('raw', um)
            burnt = p.sienna_measurement('burnt', um)
            self.assertLess(burnt['actual_rgb_contrast_ratio'], raw['actual_rgb_contrast_ratio'])
            self.assertLess(burnt['absolute_difference_retained'], raw['absolute_difference_retained'])

    def test_invalid_geometry_is_rejected(self):
        with self.assertRaises(ValueError):
            p.flow_step_summary(0.1, 0.0, 1.0)
        with self.assertRaises(ValueError):
            p.most_liftable_um(-1.0)

if __name__ == '__main__':
    unittest.main()
