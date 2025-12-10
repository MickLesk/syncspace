#!/usr/bin/env python3
"""
Migration Consolidation Script for SyncSpace
Merges all SQL migrations into a single clean schema file.
"""

import os
import re
from pathlib import Path
from collections import defaultdict

MIGRATIONS_DIR = Path("migrations")
OUTPUT_FILE = Path("migrations_consolidated/001_initial_schema.sql")

def parse_migrations():
    """Parse all migration files and extract schema information."""
    
    tables = {}  # table_name -> {columns: {}, indexes: [], constraints: []}
    table_order = []  # Track order for foreign key dependencies
    inserts = []
    other_statements = []
    
    migration_files = sorted(MIGRATIONS_DIR.glob("*.sql"))
    
    for migration_file in migration_files:
        print(f"Processing: {migration_file.name}")
        content = migration_file.read_text()
        
        # Remove comments but keep track of them
        lines = content.split('\n')
        
        # Find CREATE TABLE statements
        create_table_pattern = r'CREATE TABLE(?:\s+IF NOT EXISTS)?\s+(\w+)\s*\((.*?)\);'
        for match in re.finditer(create_table_pattern, content, re.DOTALL | re.IGNORECASE):
            table_name = match.group(1)
            table_def = match.group(2).strip()
            
            if table_name not in tables:
                tables[table_name] = {
                    'columns': {},
                    'constraints': [],
                    'indexes': [],
                    'raw_def': table_def
                }
                table_order.append(table_name)
            else:
                # Table already exists, might have different schema - use latest
                tables[table_name]['raw_def'] = table_def
        
        # Find ALTER TABLE ADD COLUMN statements
        alter_pattern = r'ALTER TABLE\s+(\w+)\s+ADD\s+COLUMN\s+(\w+)\s+([^;]+);'
        for match in re.finditer(alter_pattern, content, re.IGNORECASE):
            table_name = match.group(1)
            column_name = match.group(2)
            column_def = match.group(3).strip()
            
            if table_name in tables:
                tables[table_name]['columns'][column_name] = column_def
        
        # Find CREATE INDEX statements
        index_pattern = r'CREATE\s+(?:UNIQUE\s+)?INDEX(?:\s+IF NOT EXISTS)?\s+(\w+)\s+ON\s+(\w+)\s*\(([^)]+)\);'
        for match in re.finditer(index_pattern, content, re.IGNORECASE):
            index_name = match.group(1)
            table_name = match.group(2)
            columns = match.group(3)
            
            is_unique = 'UNIQUE' in match.group(0).upper()
            
            if table_name in tables:
                idx_def = f"CREATE {'UNIQUE ' if is_unique else ''}INDEX IF NOT EXISTS {index_name} ON {table_name}({columns})"
                if idx_def not in tables[table_name]['indexes']:
                    tables[table_name]['indexes'].append(idx_def)
        
        # Find INSERT statements (for default data)
        insert_pattern = r'INSERT\s+(?:OR\s+(?:IGNORE|REPLACE)\s+)?INTO\s+\w+[^;]+;'
        for match in re.finditer(insert_pattern, content, re.IGNORECASE):
            stmt = match.group(0).strip()
            if stmt not in inserts:
                inserts.append(stmt)
    
    return tables, table_order, inserts

def merge_table_definitions(tables):
    """Merge ALTER TABLE ADD COLUMN into CREATE TABLE definitions."""
    
    merged = {}
    
    for table_name, table_info in tables.items():
        raw_def = table_info['raw_def']
        added_columns = table_info['columns']
        
        if added_columns:
            # Parse existing columns from raw_def
            # Add new columns before closing constraints (FOREIGN KEY, etc.)
            
            # Find where constraints start
            lines = raw_def.split(',')
            column_lines = []
            constraint_lines = []
            
            for line in lines:
                line = line.strip()
                if line.upper().startswith(('FOREIGN KEY', 'PRIMARY KEY(', 'UNIQUE(', 'CHECK(')):
                    constraint_lines.append(line)
                elif line:
                    column_lines.append(line)
            
            # Add new columns
            for col_name, col_def in added_columns.items():
                # Check if column already exists
                col_exists = any(col_name.lower() in line.lower().split()[0] for line in column_lines if line.strip())
                if not col_exists:
                    column_lines.append(f"    {col_name} {col_def}")
            
            # Reconstruct table definition
            all_lines = column_lines + constraint_lines
            merged[table_name] = ',\n'.join(all_lines)
        else:
            merged[table_name] = raw_def
        
        merged[table_name] = (merged[table_name], table_info['indexes'])
    
    return merged

def generate_consolidated_sql(tables, table_order, inserts):
    """Generate the final consolidated SQL file."""
    
    output = []
    output.append("-- ============================================")
    output.append("-- SyncSpace Database Schema")
    output.append("-- Consolidated from all migrations")
    output.append(f"-- Generated: {__import__('datetime').datetime.now().isoformat()}")
    output.append("-- ============================================")
    output.append("")
    
    # Core tables first (users, files, etc.)
    priority_tables = ['users', 'files', 'folders', 'shares', 'roles', 'permissions']
    
    # Sort tables: priority first, then alphabetically
    sorted_tables = []
    for t in priority_tables:
        if t in table_order:
            sorted_tables.append(t)
    for t in table_order:
        if t not in sorted_tables:
            sorted_tables.append(t)
    
    for table_name in sorted_tables:
        if table_name not in tables:
            continue
            
        table_def, indexes = tables[table_name]
        
        output.append(f"-- Table: {table_name}")
        output.append(f"CREATE TABLE IF NOT EXISTS {table_name} (")
        output.append(f"{table_def}")
        output.append(");")
        output.append("")
        
        # Add indexes for this table
        for idx in indexes:
            output.append(f"{idx};")
        
        if indexes:
            output.append("")
    
    # Add INSERT statements at the end
    if inserts:
        output.append("-- Default Data")
        output.append("")
        for insert in inserts:
            # Only include important inserts (users, settings, etc.)
            if 'users' in insert.lower() or 'settings' in insert.lower() or 'roles' in insert.lower():
                output.append(insert)
                output.append("")
    
    return '\n'.join(output)

def main():
    print("🔄 Consolidating SyncSpace migrations...")
    
    # Create output directory
    OUTPUT_FILE.parent.mkdir(exist_ok=True)
    
    # Parse all migrations
    tables, table_order, inserts = parse_migrations()
    print(f"📊 Found {len(tables)} tables")
    
    # Merge definitions
    merged_tables = merge_table_definitions(tables)
    
    # Generate output
    sql = generate_consolidated_sql(merged_tables, table_order, inserts)
    
    # Write output
    OUTPUT_FILE.write_text(sql)
    print(f"✅ Written to {OUTPUT_FILE}")
    print(f"📝 Total lines: {len(sql.splitlines())}")

if __name__ == "__main__":
    main()
