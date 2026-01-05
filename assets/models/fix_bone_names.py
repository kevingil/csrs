# Fix Bone Names Script for Blender
#
# This script removes numeric suffixes from Mixamo bone names in the skin model
# to match standard Mixamo naming used in animation files.
#
# Example: "mixamorig:Hips_02" → "mixamorig:Hips"
# BUT: "mixamorig:LeftHandIndex4" stays as is (4 is part of the name, not a suffix)
#
# Usage:
#   1. Open your skin model in Blender (attacker_default_skin.glb)
#   2. Open Text Editor panel (Scripting workspace)
#   3. Paste or open this script
#   4. Click "Run Script"
#   5. Export as GLB to replace the original file

import bpy
import re

# Standard Mixamo bone names (these are the target names we want)
STANDARD_MIXAMO_BONES = {
    "mixamorig:Hips",
    "mixamorig:Spine",
    "mixamorig:Spine1",
    "mixamorig:Spine2",
    "mixamorig:Neck",
    "mixamorig:Head",
    "mixamorig:HeadTop_End",
    "mixamorig:LeftShoulder",
    "mixamorig:LeftArm",
    "mixamorig:LeftForeArm",
    "mixamorig:LeftHand",
    "mixamorig:LeftHandThumb1",
    "mixamorig:LeftHandThumb2",
    "mixamorig:LeftHandThumb3",
    "mixamorig:LeftHandThumb4",
    "mixamorig:LeftHandIndex1",
    "mixamorig:LeftHandIndex2",
    "mixamorig:LeftHandIndex3",
    "mixamorig:LeftHandIndex4",
    "mixamorig:LeftHandMiddle1",
    "mixamorig:LeftHandMiddle2",
    "mixamorig:LeftHandMiddle3",
    "mixamorig:LeftHandMiddle4",
    "mixamorig:LeftHandRing1",
    "mixamorig:LeftHandRing2",
    "mixamorig:LeftHandRing3",
    "mixamorig:LeftHandRing4",
    "mixamorig:LeftHandPinky1",
    "mixamorig:LeftHandPinky2",
    "mixamorig:LeftHandPinky3",
    "mixamorig:LeftHandPinky4",
    "mixamorig:RightShoulder",
    "mixamorig:RightArm",
    "mixamorig:RightForeArm",
    "mixamorig:RightHand",
    "mixamorig:RightHandThumb1",
    "mixamorig:RightHandThumb2",
    "mixamorig:RightHandThumb3",
    "mixamorig:RightHandThumb4",
    "mixamorig:RightHandIndex1",
    "mixamorig:RightHandIndex2",
    "mixamorig:RightHandIndex3",
    "mixamorig:RightHandIndex4",
    "mixamorig:RightHandMiddle1",
    "mixamorig:RightHandMiddle2",
    "mixamorig:RightHandMiddle3",
    "mixamorig:RightHandMiddle4",
    "mixamorig:RightHandRing1",
    "mixamorig:RightHandRing2",
    "mixamorig:RightHandRing3",
    "mixamorig:RightHandRing4",
    "mixamorig:RightHandPinky1",
    "mixamorig:RightHandPinky2",
    "mixamorig:RightHandPinky3",
    "mixamorig:RightHandPinky4",
    "mixamorig:LeftUpLeg",
    "mixamorig:LeftLeg",
    "mixamorig:LeftFoot",
    "mixamorig:LeftToeBase",
    "mixamorig:LeftToe_End",
    "mixamorig:RightUpLeg",
    "mixamorig:RightLeg",
    "mixamorig:RightFoot",
    "mixamorig:RightToeBase",
    "mixamorig:RightToe_End",
}

def get_standard_name(bone_name):
    """
    Try to find the standard Mixamo name for a given bone name.
    Returns the standard name if found, None otherwise.
    """
    # If it's already a standard name, return it
    if bone_name in STANDARD_MIXAMO_BONES:
        return bone_name
    
    # Try removing underscore + digits suffix
    # Pattern: ends with _XXX where X is a digit (like _02, _015, _029)
    match = re.match(r'^(.+)_(\d{2,3})$', bone_name)
    if match:
        potential_name = match.group(1)
        if potential_name in STANDARD_MIXAMO_BONES:
            return potential_name
    
    return None

def fix_bone_names():
    """Remove numeric suffixes from Mixamo bone names."""
    
    print("\n" + "=" * 60)
    print("Fix Bone Names Script")
    print("=" * 60 + "\n")
    
    renamed_count = 0
    skipped_count = 0
    already_correct = 0
    unknown_bones = []
    
    # Process all armatures in the scene
    for obj in bpy.data.objects:
        if obj.type != 'ARMATURE':
            continue
            
        print(f"Processing armature: {obj.name}")
        armature = obj.data
        
        # We need to be in edit mode to rename bones
        bpy.context.view_layer.objects.active = obj
        bpy.ops.object.mode_set(mode='EDIT')
        
        # Collect renames first (can't rename while iterating)
        renames = []
        
        for bone in armature.edit_bones:
            old_name = bone.name
            
            # Skip non-mixamo bones
            if not old_name.startswith("mixamorig:"):
                continue
            
            # Get the standard name
            standard_name = get_standard_name(old_name)
            
            if standard_name is None:
                unknown_bones.append(old_name)
                continue
            
            if old_name == standard_name:
                already_correct += 1
                continue
            
            renames.append((bone, old_name, standard_name))
        
        # Apply renames
        for bone, old_name, new_name in renames:
            # Check if target name already exists
            if new_name in armature.edit_bones and armature.edit_bones[new_name] != bone:
                print(f"  SKIP: {old_name} → {new_name} (target already exists)")
                skipped_count += 1
            else:
                bone.name = new_name
                print(f"  RENAME: {old_name} → {new_name}")
                renamed_count += 1
        
        # Return to object mode
        bpy.ops.object.mode_set(mode='OBJECT')
    
    print("\n" + "=" * 60)
    print("COMPLETE!")
    print("=" * 60)
    print(f"\nRenamed: {renamed_count} bones")
    print(f"Already correct: {already_correct} bones")
    print(f"Skipped: {skipped_count} bones (target name already exists)")
    
    if unknown_bones:
        print(f"\nUnknown bones (not renamed): {len(unknown_bones)}")
        for name in unknown_bones[:10]:
            print(f"  - {name}")
        if len(unknown_bones) > 10:
            print(f"  ... and {len(unknown_bones) - 10} more")
    
    print("\nNext steps:")
    print("  1. File → Export → glTF 2.0 (.glb)")
    print("  2. Save as: attacker_default_skin.glb")
    print("  3. Overwrite the original file in assets/models/skins/")
    
    return renamed_count

# Run the script
if __name__ == "__main__":
    fix_bone_names()
