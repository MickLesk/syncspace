#!/usr/bin/env python3
"""Defensive accessibility fixer - safe simple patterns only"""

import re
import glob

def safe_label_to_div_fix(filepath):
    """Only fix labels with text that clearly are NOT form labels"""
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()
    
    original = content
    fixes = 0
    
    # ONLY convert labels that are clearly decorative:
    # Pattern: <label class="...label-text...">Text</label> (without any associated input)
    # These are used as styled headings, not form labels
    
    #Simple one-liner labels
    patterns_to_fix = [
        (r'<label class="label">\s*<span class="label-text">', '<div class="label"><span class="label-text">'),
        (r'</span>\s*</label>', '</span></div>'),
        
        # Label-text patterns (DaisyUI labels used as decorative text)
        (r'<label\s+class="label">\s*<span\s+class="label-text-alt">',  '<div class="label"><span class="label-text-alt">'),
    ]
    
    for pattern, repl in patterns_to_fix:
        new_content = re.sub(pattern, repl, content)
        if new_content != content:
            fixes += 1
            content = new_content
    
    with open(filepath, 'w', encoding='utf-8') as f:
        f.write(content)
    
    return fixes

def fix_icon_buttons(filepath):
    """Add aria-labels to all icon-only buttons"""
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()
    
    fixes = 0
    
    # Find all <button> elements that:
    # 1. Don't have aria-label
    # 2. Only contain an <i> icon element
    
    # Pattern: <button...><i class="bi bi-X"></i></button>
    button_pattern = r'<button\s+([^>]*?)>\s*<i\s+class="bi\s+bi-([^"]+)"[^>]*>\s*</i>\s*</button>'
    
    def add_aria_label(match):
        button_attrs = match.group(1)
        icon_name = match.group(2)
        
        # Skip if aria-label already present
        if 'aria-label' in button_attrs:
            return match.group(0)
        
        # Map icon names to labels
        icon_labels = {
            'x': 'Close',
            'x-lg': 'Close',
            'trash': 'Delete',
            'trash-2': 'Delete',
            'plus': 'Add',
            'plus-lg': 'Add',
            'pencil': 'Edit',
            'pencil-fill': 'Edit',
            'play': 'Play',
            'play-fill': 'Play',
            'pause': 'Pause',
            'pause-fill': 'Pause',
            'download': 'Download',
            'upload': 'Upload',
            'search': 'Search',
            'gear': 'Settings',
            'save': 'Save',
            'eye': 'View',
            'share': 'Share',
            'link': 'Link',
        }
        
        label = icon_labels.get(icon_name, 'Button')
        
        # Insert aria-label at the beginning
        return f'<button aria-label="{label}" {button_attrs}><i class="bi bi-{icon_name}" aria-hidden="true"></i></button>'
    
    new_content = re.sub(button_pattern, add_aria_label, content, flags=re.DOTALL)
    if new_content != content:
        fixes += 1
        content = new_content
    
    with open(filepath, 'w', encoding='utf-8') as f:
        f.write(content)
    
    return fixes

def fix_modal_backdrops_simple(filepath):
    """Add keyboard support to modal backdrop divs"""
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()
    
    original = content
    fixes = 0
    
    # Simple pattern: <div class="modal-backdrop" onclick={close}></div>
    patterns = [
        # Pattern 1: onclick with function name
        (r'<div\s+class="modal-backdrop"\s+onclick=\{([a-zA-Z0-9_]+)\}\s*></div>',
         r'<div class="modal-backdrop" role="dialog" tabindex="0" onclick={\\1} onkeydown={(e) => e.key === "Escape" && \\1()}></div>'),
        
        # Pattern 2: onclick with arrow function
        (r'<div\s+class="modal-backdrop"\s+onclick=\(\) => \(([^)]+)\)\s*></div>',
         r'<div class="modal-backdrop" role="dialog" tabindex="0" onclick={() => (\\1)} onkeydown={(e) => e.key === "Escape" && (\\1)}></div>'),
    ]
    
    for pattern, repl in patterns:
        new_content = re.sub(pattern, repl, content)
        if new_content != content:
            fixes += 1
            content = new_content
    
    with open(filepath, 'w', encoding='utf-8') as f:
        f.write(content)
    
    return fixes

def main():
    print("🔧 Running defensive accessibility fixes...")
    
    svelte_files = glob.glob(
        r'c:\Users\LeskowitzMickey\Documents\GitHub\syncspace\frontend\src\**\*.svelte',
        recursive=True
    )
    
    total_fixes = 0
    file_results = {}
    
    for filepath in svelte_files:
        fixes = 0
        fixes += safe_label_to_div_fix(filepath)
        fixes += fix_icon_buttons(filepath)
        fixes += fix_modal_backdrops_simple(filepath)
        
        if fixes > 0:
            filename = filepath.split('\\')[-1]
            file_results[filename] = fixes
            total_fixes += fixes
    
    print(f"\n✅ Fixes applied:")
    for filename, count in sorted(file_results.items(), key=lambda x: x[1], reverse=True):
        print(f"  {filename}: {count}")
    
    print(f"\n📊 Total: {total_fixes} fixes")

if __name__ == "__main__":
    main()
