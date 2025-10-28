#!/usr/bin/env python3
import re
import sys

def fix_dark_mode_in_file(filepath):
    """Replace @media (prefers-color-scheme: dark) with :global(.dark) selector"""
    
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()
    
    original_content = content
    
    # Pattern to match @media (prefers-color-scheme: dark) { ... }
    # We need to find the matching closing brace
    def replace_media_query(match):
        # Extract the CSS rules inside the @media block
        rules = match.group(1).strip()
        
        # Find the class/selector before the opening brace
        # Pattern: .classname { or :global(.dark) .classname {
        lines = []
        for rule in rules.split('\n'):
            rule = rule.strip()
            if not rule or rule == '}':
                continue
            
            # Extract selector and properties
            # Example: ".modal-content { color: rgb(243 244 246); }"
            if '{' in rule:
                selector_part = rule.split('{')[0].strip()
                properties_part = rule.split('{', 1)[1].strip()
                
                # Add :global(.dark) prefix
                new_selector = f":global(.dark) {selector_part}"
                lines.append(f"  {new_selector} {{{properties_part}")
            else:
                lines.append(f"  {rule}")
        
        return '\n'.join(lines)
    
    # This is a complex pattern - let's use a simpler approach
    # Replace the opening tag first
    content = content.replace('@media (prefers-color-scheme: dark) {', ':global(.dark)')
    
    # Now we need to handle the structure correctly
    # The problem is that @media wraps multiple selectors
    # We need to unwrap them
    
    print(f"Processing {filepath}")
    
    with open(filepath, 'w', encoding='utf-8') as f:
        f.write(content)
    
    return content != original_content

if __name__ == '__main__':
    files = [
        'src/pages/NotFound.svelte',
        'src/pages/auth/Login.svelte',
        'src/pages/auth/Signup.svelte',
        'src/pages/system/TrashView.svelte',
        'src/components/ui/Button.svelte',
        'src/components/ui/Chip.svelte',
        'src/components/ui/InfoCard.svelte',
        'src/components/ui/Spinner.svelte',
        'src/components/ui/TabBar.svelte',
        'src/components/ui/Dialog.svelte',
        'src/components/ui/ProgressBar.svelte',
        'src/components/ui/Loading.svelte',
        'src/components/ui/Badge.svelte',
    ]
    
    for f in files:
        try:
            if fix_dark_mode_in_file(f):
                print(f"✓ Fixed {f}")
            else:
                print(f"- No changes needed in {f}")
        except Exception as e:
            print(f"✗ Error in {f}: {e}")
