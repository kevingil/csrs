# Player Animation Generator Script for Blender (Mixamo Compatible)
# 
# This script creates animations compatible with Mixamo-rigged character models.
# All animations use the standard Mixamo bone naming convention.
#
# Usage:
#   1. Open Blender
#   2. Open Text Editor panel (Scripting workspace)
#   3. Paste or open this script
#   4. Click "Run Script"
#   5. Export as GLB: File -> Export -> glTF 2.0 (.glb/.gltf)
#
# The script will create:
#   - Armature with Mixamo bone hierarchy
#   - 16 animation actions with placeholder keyframes

import bpy
import math
from mathutils import Vector, Euler

# =============================================================================
# Configuration
# =============================================================================

FPS = 24  # Animation framerate

# Animation definitions: (name, duration_seconds, is_looping)
ANIMATIONS = [
    # Lower body (indices 0-4)
    ("lower_idle", 2.0, True),
    ("lower_walk", 1.0, True),
    ("lower_run", 0.6, True),
    ("lower_crouch_idle", 2.0, True),
    ("lower_crouch_walk", 1.0, True),
    # Rifle (indices 5-7)
    ("rifle_idle", 2.0, True),
    ("rifle_reload", 2.5, False),
    ("rifle_fire", 0.1, False),
    # Pistol (indices 8-10)
    ("pistol_idle", 2.0, True),
    ("pistol_reload", 1.8, False),
    ("pistol_fire", 0.15, False),
    # Sniper (indices 11-13)
    ("sniper_idle", 2.0, True),
    ("sniper_reload", 3.5, False),
    ("sniper_fire", 1.5, False),
    # Knife (indices 14-15)
    ("knife_idle", 2.0, True),
    ("knife_attack", 0.5, False),
]

# =============================================================================
# Mixamo Bone Definitions
# =============================================================================
# Format: (name, parent, head_offset, tail_offset, roll)
# Offsets are relative to parent's tail (or origin for root)
# All measurements in meters (Blender units)

BONE_DEFINITIONS = [
    # Root/Hips
    ("mixamorig:Hips", None, (0, 0, 0), (0, 0, 0.1), 0),
    
    # Spine chain
    ("mixamorig:Spine", "mixamorig:Hips", (0, 0, 0), (0, 0, 0.12), 0),
    ("mixamorig:Spine1", "mixamorig:Spine", (0, 0, 0), (0, 0, 0.12), 0),
    ("mixamorig:Spine2", "mixamorig:Spine1", (0, 0, 0), (0, 0, 0.12), 0),
    
    # Head
    ("mixamorig:Neck", "mixamorig:Spine2", (0, 0, 0), (0, 0, 0.08), 0),
    ("mixamorig:Head", "mixamorig:Neck", (0, 0, 0), (0, 0, 0.2), 0),
    ("mixamorig:HeadTop_End", "mixamorig:Head", (0, 0, 0), (0, 0, 0.1), 0),
    
    # Left arm
    ("mixamorig:LeftShoulder", "mixamorig:Spine2", (0.05, 0, -0.02), (0.12, 0, 0), 0),
    ("mixamorig:LeftArm", "mixamorig:LeftShoulder", (0, 0, 0), (0.28, 0, 0), math.pi),
    ("mixamorig:LeftForeArm", "mixamorig:LeftArm", (0, 0, 0), (0.25, 0, 0), math.pi),
    ("mixamorig:LeftHand", "mixamorig:LeftForeArm", (0, 0, 0), (0.08, 0, 0), 0),
    
    # Left hand fingers
    ("mixamorig:LeftHandThumb1", "mixamorig:LeftHand", (-0.02, 0.02, 0), (0.03, 0.02, 0), 0),
    ("mixamorig:LeftHandThumb2", "mixamorig:LeftHandThumb1", (0, 0, 0), (0.025, 0, 0), 0),
    ("mixamorig:LeftHandThumb3", "mixamorig:LeftHandThumb2", (0, 0, 0), (0.02, 0, 0), 0),
    ("mixamorig:LeftHandThumb4", "mixamorig:LeftHandThumb3", (0, 0, 0), (0.015, 0, 0), 0),
    
    ("mixamorig:LeftHandIndex1", "mixamorig:LeftHand", (0, 0, 0), (0.04, 0.01, 0), 0),
    ("mixamorig:LeftHandIndex2", "mixamorig:LeftHandIndex1", (0, 0, 0), (0.025, 0, 0), 0),
    ("mixamorig:LeftHandIndex3", "mixamorig:LeftHandIndex2", (0, 0, 0), (0.02, 0, 0), 0),
    ("mixamorig:LeftHandIndex4", "mixamorig:LeftHandIndex3", (0, 0, 0), (0.015, 0, 0), 0),
    
    ("mixamorig:LeftHandMiddle1", "mixamorig:LeftHand", (0, 0, 0), (0.045, 0, 0), 0),
    ("mixamorig:LeftHandMiddle2", "mixamorig:LeftHandMiddle1", (0, 0, 0), (0.03, 0, 0), 0),
    ("mixamorig:LeftHandMiddle3", "mixamorig:LeftHandMiddle2", (0, 0, 0), (0.022, 0, 0), 0),
    ("mixamorig:LeftHandMiddle4", "mixamorig:LeftHandMiddle3", (0, 0, 0), (0.015, 0, 0), 0),
    
    ("mixamorig:LeftHandRing1", "mixamorig:LeftHand", (0, 0, 0), (0.04, -0.01, 0), 0),
    ("mixamorig:LeftHandRing2", "mixamorig:LeftHandRing1", (0, 0, 0), (0.028, 0, 0), 0),
    ("mixamorig:LeftHandRing3", "mixamorig:LeftHandRing2", (0, 0, 0), (0.02, 0, 0), 0),
    ("mixamorig:LeftHandRing4", "mixamorig:LeftHandRing3", (0, 0, 0), (0.015, 0, 0), 0),
    
    ("mixamorig:LeftHandPinky1", "mixamorig:LeftHand", (0, 0, 0), (0.035, -0.02, 0), 0),
    ("mixamorig:LeftHandPinky2", "mixamorig:LeftHandPinky1", (0, 0, 0), (0.022, 0, 0), 0),
    ("mixamorig:LeftHandPinky3", "mixamorig:LeftHandPinky2", (0, 0, 0), (0.018, 0, 0), 0),
    ("mixamorig:LeftHandPinky4", "mixamorig:LeftHandPinky3", (0, 0, 0), (0.015, 0, 0), 0),
    
    # Right arm
    ("mixamorig:RightShoulder", "mixamorig:Spine2", (-0.05, 0, -0.02), (-0.12, 0, 0), 0),
    ("mixamorig:RightArm", "mixamorig:RightShoulder", (0, 0, 0), (-0.28, 0, 0), -math.pi),
    ("mixamorig:RightForeArm", "mixamorig:RightArm", (0, 0, 0), (-0.25, 0, 0), -math.pi),
    ("mixamorig:RightHand", "mixamorig:RightForeArm", (0, 0, 0), (-0.08, 0, 0), 0),
    
    # Right hand fingers
    ("mixamorig:RightHandThumb1", "mixamorig:RightHand", (0.02, 0.02, 0), (-0.03, 0.02, 0), 0),
    ("mixamorig:RightHandThumb2", "mixamorig:RightHandThumb1", (0, 0, 0), (-0.025, 0, 0), 0),
    ("mixamorig:RightHandThumb3", "mixamorig:RightHandThumb2", (0, 0, 0), (-0.02, 0, 0), 0),
    ("mixamorig:RightHandThumb4", "mixamorig:RightHandThumb3", (0, 0, 0), (-0.015, 0, 0), 0),
    
    ("mixamorig:RightHandIndex1", "mixamorig:RightHand", (0, 0, 0), (-0.04, 0.01, 0), 0),
    ("mixamorig:RightHandIndex2", "mixamorig:RightHandIndex1", (0, 0, 0), (-0.025, 0, 0), 0),
    ("mixamorig:RightHandIndex3", "mixamorig:RightHandIndex2", (0, 0, 0), (-0.02, 0, 0), 0),
    ("mixamorig:RightHandIndex4", "mixamorig:RightHandIndex3", (0, 0, 0), (-0.015, 0, 0), 0),
    
    ("mixamorig:RightHandMiddle1", "mixamorig:RightHand", (0, 0, 0), (-0.045, 0, 0), 0),
    ("mixamorig:RightHandMiddle2", "mixamorig:RightHandMiddle1", (0, 0, 0), (-0.03, 0, 0), 0),
    ("mixamorig:RightHandMiddle3", "mixamorig:RightHandMiddle2", (0, 0, 0), (-0.022, 0, 0), 0),
    ("mixamorig:RightHandMiddle4", "mixamorig:RightHandMiddle3", (0, 0, 0), (-0.015, 0, 0), 0),
    
    ("mixamorig:RightHandRing1", "mixamorig:RightHand", (0, 0, 0), (-0.04, -0.01, 0), 0),
    ("mixamorig:RightHandRing2", "mixamorig:RightHandRing1", (0, 0, 0), (-0.028, 0, 0), 0),
    ("mixamorig:RightHandRing3", "mixamorig:RightHandRing2", (0, 0, 0), (-0.02, 0, 0), 0),
    ("mixamorig:RightHandRing4", "mixamorig:RightHandRing3", (0, 0, 0), (-0.015, 0, 0), 0),
    
    ("mixamorig:RightHandPinky1", "mixamorig:RightHand", (0, 0, 0), (-0.035, -0.02, 0), 0),
    ("mixamorig:RightHandPinky2", "mixamorig:RightHandPinky1", (0, 0, 0), (-0.022, 0, 0), 0),
    ("mixamorig:RightHandPinky3", "mixamorig:RightHandPinky2", (0, 0, 0), (-0.018, 0, 0), 0),
    ("mixamorig:RightHandPinky4", "mixamorig:RightHandPinky3", (0, 0, 0), (-0.015, 0, 0), 0),
    
    # Left leg
    ("mixamorig:LeftUpLeg", "mixamorig:Hips", (0.1, 0, 0), (0.1, 0, -0.45), 0),
    ("mixamorig:LeftLeg", "mixamorig:LeftUpLeg", (0, 0, 0), (0, 0, -0.42), 0),
    ("mixamorig:LeftFoot", "mixamorig:LeftLeg", (0, 0, 0), (0, -0.12, 0.03), 0),
    ("mixamorig:LeftToeBase", "mixamorig:LeftFoot", (0, 0, 0), (0, -0.08, 0), 0),
    ("mixamorig:LeftToe_End", "mixamorig:LeftToeBase", (0, 0, 0), (0, -0.04, 0), 0),
    
    # Right leg
    ("mixamorig:RightUpLeg", "mixamorig:Hips", (-0.1, 0, 0), (-0.1, 0, -0.45), 0),
    ("mixamorig:RightLeg", "mixamorig:RightUpLeg", (0, 0, 0), (0, 0, -0.42), 0),
    ("mixamorig:RightFoot", "mixamorig:RightLeg", (0, 0, 0), (0, -0.12, 0.03), 0),
    ("mixamorig:RightToeBase", "mixamorig:RightFoot", (0, 0, 0), (0, -0.08, 0), 0),
    ("mixamorig:RightToe_End", "mixamorig:RightToeBase", (0, 0, 0), (0, -0.04, 0), 0),
]

# Lower body bones (for locomotion animations)
LOWER_BODY_BONES = [
    "mixamorig:Hips",
    "mixamorig:Spine",
    "mixamorig:LeftUpLeg", "mixamorig:LeftLeg", "mixamorig:LeftFoot", "mixamorig:LeftToeBase",
    "mixamorig:RightUpLeg", "mixamorig:RightLeg", "mixamorig:RightFoot", "mixamorig:RightToeBase",
]

# Upper body bones (for weapon animations)
UPPER_BODY_BONES = [
    "mixamorig:Spine1", "mixamorig:Spine2",
    "mixamorig:Neck", "mixamorig:Head",
    "mixamorig:LeftShoulder", "mixamorig:LeftArm", "mixamorig:LeftForeArm", "mixamorig:LeftHand",
    "mixamorig:LeftHandThumb1", "mixamorig:LeftHandThumb2", "mixamorig:LeftHandThumb3",
    "mixamorig:LeftHandIndex1", "mixamorig:LeftHandIndex2", "mixamorig:LeftHandIndex3",
    "mixamorig:LeftHandMiddle1", "mixamorig:LeftHandMiddle2", "mixamorig:LeftHandMiddle3",
    "mixamorig:LeftHandRing1", "mixamorig:LeftHandRing2", "mixamorig:LeftHandRing3",
    "mixamorig:LeftHandPinky1", "mixamorig:LeftHandPinky2", "mixamorig:LeftHandPinky3",
    "mixamorig:RightShoulder", "mixamorig:RightArm", "mixamorig:RightForeArm", "mixamorig:RightHand",
    "mixamorig:RightHandThumb1", "mixamorig:RightHandThumb2", "mixamorig:RightHandThumb3",
    "mixamorig:RightHandIndex1", "mixamorig:RightHandIndex2", "mixamorig:RightHandIndex3",
    "mixamorig:RightHandMiddle1", "mixamorig:RightHandMiddle2", "mixamorig:RightHandMiddle3",
    "mixamorig:RightHandRing1", "mixamorig:RightHandRing2", "mixamorig:RightHandRing3",
    "mixamorig:RightHandPinky1", "mixamorig:RightHandPinky2", "mixamorig:RightHandPinky3",
]


# =============================================================================
# Helper Functions
# =============================================================================

def clear_scene():
    """Remove all objects from the scene."""
    bpy.ops.object.select_all(action='SELECT')
    bpy.ops.object.delete(use_global=False)
    
    # Clear all actions
    for action in bpy.data.actions:
        bpy.data.actions.remove(action)


def create_armature():
    """Create the player armature with Mixamo bone hierarchy."""
    # Create armature data
    armature_data = bpy.data.armatures.new("Armature")
    armature_obj = bpy.data.objects.new("Armature", armature_data)
    
    # Link to scene
    bpy.context.collection.objects.link(armature_obj)
    bpy.context.view_layer.objects.active = armature_obj
    armature_obj.select_set(True)
    
    # Enter edit mode to create bones
    bpy.ops.object.mode_set(mode='EDIT')
    
    # Track bone positions for parenting
    bone_tails = {}
    
    for bone_name, parent_name, head_offset, tail_offset, roll in BONE_DEFINITIONS:
        bone = armature_data.edit_bones.new(bone_name)
        
        # Calculate head position
        if parent_name is None:
            # Root bone starts at origin, raised to pelvis height
            base_pos = Vector((0, 0, 1.0))  # ~1m up (pelvis height)
        else:
            # Start from parent's tail
            base_pos = bone_tails[parent_name].copy()
        
        head_pos = base_pos + Vector(head_offset)
        tail_pos = head_pos + Vector(tail_offset)
        
        bone.head = head_pos
        bone.tail = tail_pos
        bone.roll = roll
        
        # Store tail position for children
        bone_tails[bone_name] = tail_pos
        
        # Set parent
        if parent_name is not None:
            bone.parent = armature_data.edit_bones[parent_name]
            bone.use_connect = (head_offset == (0, 0, 0))
    
    # Return to object mode
    bpy.ops.object.mode_set(mode='OBJECT')
    
    return armature_obj


def create_animation_action(armature_obj, anim_name, duration_seconds, is_lower_body):
    """Create an animation action with placeholder keyframes."""
    # Calculate frame count
    frame_count = max(2, int(duration_seconds * FPS))
    
    # Determine which bones to animate
    if is_lower_body:
        bones_to_animate = LOWER_BODY_BONES
    else:
        bones_to_animate = UPPER_BODY_BONES
    
    # Enter pose mode
    bpy.context.view_layer.objects.active = armature_obj
    bpy.ops.object.mode_set(mode='POSE')
    
    # Create animation data if needed
    if armature_obj.animation_data is None:
        armature_obj.animation_data_create()
    
    # Create new action and assign it
    action = bpy.data.actions.new(name=anim_name)
    armature_obj.animation_data.action = action
    
    # Create keyframes for each bone using keyframe_insert
    for bone_name in bones_to_animate:
        if bone_name not in armature_obj.pose.bones:
            continue
            
        pose_bone = armature_obj.pose.bones[bone_name]
        
        # Ensure bone uses quaternion rotation
        pose_bone.rotation_mode = 'QUATERNION'
        
        # Reset to identity quaternion
        pose_bone.rotation_quaternion = (1, 0, 0, 0)
        
        # Keyframe at start (frame 1)
        bpy.context.scene.frame_set(1)
        pose_bone.keyframe_insert(data_path="rotation_quaternion", frame=1)
        
        # Keyframe at middle with slight offset (shows animation exists)
        if frame_count > 4:
            mid_frame = frame_count // 2
            bpy.context.scene.frame_set(mid_frame)
            # Add tiny rotation to show animation is present
            pose_bone.rotation_quaternion = (0.998, 0.01, 0.01, 0.01)
            pose_bone.keyframe_insert(data_path="rotation_quaternion", frame=mid_frame)
        
        # Keyframe at end (back to identity)
        bpy.context.scene.frame_set(frame_count)
        pose_bone.rotation_quaternion = (1, 0, 0, 0)
        pose_bone.keyframe_insert(data_path="rotation_quaternion", frame=frame_count)
    
    # Reset frame
    bpy.context.scene.frame_set(1)
    
    # Return to object mode
    bpy.ops.object.mode_set(mode='OBJECT')
    
    return action


def create_all_animations(armature_obj):
    """Create all 16 animation actions."""
    actions = []
    
    for idx, (anim_name, duration, is_looping) in enumerate(ANIMATIONS):
        # Lower body animations are indices 0-4
        is_lower_body = anim_name.startswith("lower_")
        
        print(f"Creating animation {idx}: {anim_name} ({duration}s)")
        action = create_animation_action(armature_obj, anim_name, duration, is_lower_body)
        actions.append(action)
    
    # Clear the active action
    if armature_obj.animation_data:
        armature_obj.animation_data.action = None
    
    return actions


def setup_nla_tracks(armature_obj, actions):
    """
    Push all actions to NLA tracks for proper export ordering.
    Actions are ordered bottom-to-top (first action = bottom track).
    """
    if armature_obj.animation_data is None:
        armature_obj.animation_data_create()
    
    # Clear existing NLA tracks
    for track in armature_obj.animation_data.nla_tracks:
        armature_obj.animation_data.nla_tracks.remove(track)
    
    # Create NLA tracks in reverse order so first action is at bottom
    for idx, action in enumerate(reversed(actions)):
        track = armature_obj.animation_data.nla_tracks.new()
        track.name = action.name
        
        # Add the action as a strip
        strip = track.strips.new(action.name, start=1, action=action)
        strip.name = action.name
        
        # Mute so it doesn't play by default
        track.mute = True
    
    print(f"\nCreated {len(actions)} NLA tracks for export ordering")


# =============================================================================
# Main Execution
# =============================================================================

def main():
    """Main entry point."""
    print("\n" + "=" * 60)
    print("Player Animation Generator (Mixamo Compatible)")
    print("=" * 60 + "\n")
    
    # Set scene framerate
    bpy.context.scene.render.fps = FPS
    
    # Clear existing objects
    print("Clearing scene...")
    clear_scene()
    
    # Create armature
    print("Creating armature with Mixamo bone hierarchy...")
    armature = create_armature()
    print(f"  Created {len(armature.data.bones)} bones")
    
    # Create animations
    print("\nCreating animations...")
    actions = create_all_animations(armature)
    
    # Setup NLA tracks for export ordering
    print("\nSetting up NLA tracks for GLB export...")
    setup_nla_tracks(armature, actions)
    
    # Select armature
    bpy.ops.object.select_all(action='DESELECT')
    armature.select_set(True)
    bpy.context.view_layer.objects.active = armature
    
    print("\n" + "=" * 60)
    print("COMPLETE!")
    print("=" * 60)
    print(f"\nCreated:")
    print(f"  - Armature with {len(armature.data.bones)} Mixamo-compatible bones")
    print(f"  - {len(actions)} animations:")
    for idx, (name, duration, _) in enumerate(ANIMATIONS):
        print(f"      [{idx:2d}] {name} ({duration}s)")
    
    print("\nMixamo bone naming convention used:")
    print("  - mixamorig:Hips (root)")
    print("  - mixamorig:Spine, Spine1, Spine2")
    print("  - mixamorig:LeftArm, RightArm, etc.")
    
    print("\nNext steps:")
    print("  1. Refine animations in Pose mode")
    print("  2. Export: File -> Export -> glTF 2.0 (.glb)")
    print("     - Format: glTF Binary (.glb)")
    print("     - Animation mode: Actions")
    print("     - Enable: Group by NLA Track")
    print("  3. Save as: player_animations.glb")


# Run the script
if __name__ == "__main__":
    main()
