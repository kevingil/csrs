"""
Blender script to fix skin model for animation compatibility.

The problem:
- Animation file has hierarchy: Armature -> mixamorig:Hips -> bones
- Skin model has hierarchy: Sketchfab_model -> ... -> _rootJoint -> mixamorig:Hips -> bones

Bevy matches animations by full path names, so these don't work together.

Solution:
This script simplifies the skin model hierarchy and renames the armature
to match the animation file's structure.

USAGE:
1. Open Blender
2. File -> Import -> glTF 2.0 -> select attacker_default_skin.glb
3. Open Scripting workspace  
4. Open this script and run it
5. Export: File -> Export -> glTF 2.0 (.glb) -> Overwrite the original

IMPORTANT EXPORT SETTINGS:
- Format: glTF Binary (.glb)
- Include: Selected Objects (select armature and mesh before export)
- Transform: +Y Up
- Data -> Mesh: Apply Modifiers: ON
"""

import bpy
import mathutils


def main():
    """Main function to fix skin model hierarchy."""
    
    print("\n" + "=" * 60)
    print("FIXING SKIN MODEL FOR BEVY ANIMATION COMPATIBILITY")
    print("=" * 60)
    
    # Step 1: Find the armature (look for the one with Mixamo bones)
    armature_obj = None
    for obj in bpy.data.objects:
        if obj.type == 'ARMATURE':
            # Check if it has mixamorig bones
            has_mixamo_bones = any('mixamorig' in bone.name for bone in obj.data.bones)
            if has_mixamo_bones:
                armature_obj = obj
                break
    
    if not armature_obj:
        print("ERROR: No armature with Mixamo bones found!")
        print("Make sure you've imported the skin model first.")
        return False
    
    print(f"Found armature: {armature_obj.name}")
    print(f"Current parent: {armature_obj.parent.name if armature_obj.parent else 'None'}")
    
    # Step 2: Find all meshes that use this armature
    meshes = []
    for obj in bpy.data.objects:
        if obj.type == 'MESH':
            for mod in obj.modifiers:
                if mod.type == 'ARMATURE' and mod.object == armature_obj:
                    meshes.append(obj)
                    break
    
    print(f"Found {len(meshes)} mesh(es) using this armature")
    
    # Step 3: Store world transforms
    armature_world_matrix = armature_obj.matrix_world.copy()
    mesh_world_matrices = {mesh.name: mesh.matrix_world.copy() for mesh in meshes}
    
    # Step 4: Unparent everything and move to world origin
    bpy.ops.object.select_all(action='DESELECT')
    
    # Unparent armature
    if armature_obj.parent:
        armature_obj.select_set(True)
        bpy.context.view_layer.objects.active = armature_obj
        bpy.ops.object.parent_clear(type='CLEAR_KEEP_TRANSFORM')
        armature_obj.select_set(False)
    
    # Unparent meshes
    for mesh in meshes:
        if mesh.parent:
            mesh.select_set(True)
            bpy.context.view_layer.objects.active = mesh
            bpy.ops.object.parent_clear(type='CLEAR_KEEP_TRANSFORM')
            mesh.select_set(False)
    
    # Step 5: Rename armature to "Armature" (matches animation file)
    old_armature_name = armature_obj.name
    armature_obj.name = "Armature"
    armature_obj.data.name = "Armature"
    print(f"Renamed armature: {old_armature_name} -> Armature")
    
    # Step 6: Check if _rootJoint is a bone or the armature itself
    # In GLTF, _rootJoint might be a bone inside the armature
    root_bone_name = None
    for bone in armature_obj.data.bones:
        if bone.parent is None:
            root_bone_name = bone.name
            print(f"Root bone is: {bone.name}")
            break
    
    # Step 7: If root bone is _rootJoint, we need to consider if we should rename it
    # For Bevy compatibility, the animation target path should match
    # Animation file might use: Armature/mixamorig:Hips
    # Our skin has: Armature/_rootJoint/mixamorig:Hips (after renaming)
    #
    # We have two options:
    # A) Remove _rootJoint bone and make mixamorig:Hips the root
    # B) Rename _rootJoint to something the animation expects
    #
    # Since the animation file has mixamorig:Hips directly under Armature,
    # we should try to match that structure.
    
    if root_bone_name == '_rootJoint':
        print("\nWARNING: Root bone is '_rootJoint' which might not match animations.")
        print("The animation file expects 'mixamorig:Hips' directly under 'Armature'.")
        print("\nTo fix this, you have two options:")
        print("1. Re-export animations with _rootJoint included in the skeleton")
        print("2. Edit this skin model to remove the _rootJoint bone")
        print("\nAttempting option 2: Removing _rootJoint bone...")
        
        # Enter edit mode to modify bones
        bpy.ops.object.select_all(action='DESELECT')
        armature_obj.select_set(True)
        bpy.context.view_layer.objects.active = armature_obj
        bpy.ops.object.mode_set(mode='EDIT')
        
        # Find _rootJoint and its child (should be mixamorig:Hips)
        edit_bones = armature_obj.data.edit_bones
        root_joint = edit_bones.get('_rootJoint')
        
        if root_joint:
            # Get the child bone (should be mixamorig:Hips)
            if root_joint.children:
                hips_bone = root_joint.children[0]
                
                # Store the hips bone's world-space head position
                # We'll need to adjust it after removing the parent
                hips_head = hips_bone.head.copy()
                
                # Unparent the hips bone
                hips_bone.parent = None
                
                # Delete _rootJoint
                edit_bones.remove(root_joint)
                print("Removed _rootJoint bone, mixamorig:Hips is now the root")
        
        bpy.ops.object.mode_set(mode='OBJECT')
    
    # Step 8: Reset transforms
    armature_obj.location = (0, 0, 0)
    armature_obj.rotation_euler = (0, 0, 0)
    armature_obj.scale = (1, 1, 1)
    
    for mesh in meshes:
        mesh.location = (0, 0, 0)
        mesh.rotation_euler = (0, 0, 0)
        mesh.scale = (1, 1, 1)
    
    # Step 9: Delete all the empty wrapper objects
    objects_to_delete = []
    for obj in bpy.data.objects:
        if obj.type == 'EMPTY':
            objects_to_delete.append(obj)
    
    for obj in objects_to_delete:
        print(f"Deleting empty: {obj.name}")
        bpy.data.objects.remove(obj, do_unlink=True)
    
    # Step 10: Print final structure
    print("\n" + "=" * 40)
    print("FINAL SCENE STRUCTURE")
    print("=" * 40)
    
    for obj in bpy.data.objects:
        if obj.parent is None:
            print_tree(obj, 0)
    
    # Print bone hierarchy
    print("\nBone hierarchy:")
    for bone in armature_obj.data.bones:
        if bone.parent is None:
            print_bone_tree(bone, 0)
    
    print("\n" + "=" * 60)
    print("SUCCESS! Now export the model:")
    print("1. Select the Armature and all mesh objects")
    print("2. File -> Export -> glTF 2.0 (.glb)")
    print("3. Enable 'Selected Objects' in Include section")
    print("4. Overwrite: assets/models/skins/attacker_default_skin.glb")
    print("=" * 60)
    
    return True


def print_tree(obj, depth):
    """Print object hierarchy tree."""
    indent = "  " * depth
    print(f"{indent}- {obj.name} ({obj.type})")
    for child in obj.children:
        print_tree(child, depth + 1)


def print_bone_tree(bone, depth):
    """Print bone hierarchy tree."""
    indent = "  " * depth
    print(f"{indent}- {bone.name}")
    for child in bone.children:
        print_bone_tree(child, depth + 1)


if __name__ == "__main__":
    main()

