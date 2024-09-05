// Copyright (c) Choreo contributors

package choreo.trajectory;

import edu.wpi.first.math.MathUtil;
import edu.wpi.first.math.geometry.Pose2d;
import edu.wpi.first.math.geometry.Rotation2d;
import edu.wpi.first.math.kinematics.ChassisSpeeds;
import choreo.util.AllianceFlipUtil;

/** A single robot sample in a ChoreoTrajectory. */
public class DiffySample implements TrajSample<DiffySample> {
  /** The timestamp of this sample, relative to the beginning of the trajectory. */
  public final double timestamp;

  /** The X position of the sample in meters. */
  public final double x;

  /** The Y position of the sample in meters. */
  public final double y;

  /** The heading of the sample in radians, with 0 being in the +X direction. */
  public final double heading;

  /** The velocity of the left side in m/s. */
  public final double vl;

  /** The velocity of the right side in m/s. */
  public final double vr;

  /** The acceleration of the left side in m/s^2. */
  public final double al;

  /** The acceleration of the right side in m/s^2. */
  public final double ar;

  /** The force of the left side in Newtons. */
  public final double fl;

  /** The force of the right side in Newtons. */
  public final double fr;

  /**
   * Constructs a DiffySample with the specified parameters.
   *
   * @param timestamp The timestamp of this sample.
   * @param x The X position of the sample in meters.
   * @param y The Y position of the sample in meters.
   * @param heading The heading of the sample in radians, with 0 being in the +X direction.
   * @param vl The velocity of the left side in m/s.
   * @param vr The velocity of the right side in m/s.
   * @param al The acceleration of the left side in m/s^2.
   * @param ar The acceleration of the right side in m/s^2.
   * @param fl The force of the left side in Newtons.
   * @param fr The force of the right side in Newtons.
   */
  public DiffySample(
      double timestamp,
      double x,
      double y,
      double heading,
      double vl,
      double vr,
      double al,
      double ar,
      double fl,
      double fr) {
    this.timestamp = timestamp;
    this.x = x;
    this.y = y;
    this.heading = heading;
    this.vl = vl;
    this.vr = vr;
    this.al = al;
    this.ar = ar;
    this.fl = fl;
    this.fr = fr;
  }


  @Override
  public double getTimestamp() {
    return timestamp;
  }

  @Override
  public Pose2d getPose() {
    return new Pose2d(x, y, Rotation2d.fromRadians(heading));
  }


  public ChassisSpeeds getChassisSpeeds() {
    // TODO: Implement getChassisSpeeds
    return new ChassisSpeeds();
  }

  @Override
  public DiffySample interpolate(DiffySample endValue, double timestamp) {
    double scale = (timestamp - this.timestamp) / (endValue.timestamp - this.timestamp);
    var interp_pose = getPose().interpolate(endValue.getPose(), scale);

    return new DiffySample(
        MathUtil.interpolate(this.timestamp, endValue.timestamp, scale),
        interp_pose.getX(),
        interp_pose.getY(),
        interp_pose.getRotation().getRadians(),
        MathUtil.interpolate(this.vl, endValue.vl, scale),
        MathUtil.interpolate(this.vr, endValue.vr, scale),
        MathUtil.interpolate(this.al, endValue.al, scale),
        MathUtil.interpolate(this.ar, endValue.ar, scale),
        MathUtil.interpolate(this.fl, endValue.fl, scale),
        MathUtil.interpolate(this.fr, endValue.fr, scale));
  }

  public DiffySample flipped() {
    switch (AllianceFlipUtil.getFlippingType()) {
      // TODO: Implement flipping
      default:
        return this;
    }
  }

  public DiffySample offsetBy(double timestampOffset) {
    return new DiffySample(
        timestamp + timestampOffset,
        x,
        y,
        heading,
        vl,
        vr,
        al,
        ar,
        fl,
        fr);
  }
}
