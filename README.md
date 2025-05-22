# CADAC

## Overview
CADAC (Command-line Application for Data Analysis and Cataloging) is a data transformation and cataloging tool designed to help data teams transform, test, catalog, and share data within their organizations. It serves as an alternative to dbt (data build tool) with a focus on pure SQL and robust parsing using tree-sitter.

## Core Requirements
1. Parse and analyze SQL queries to extract metadata using tree-sitter
2. Discover SQL models from directories and build a model catalog
3. Track dependencies between models for proper execution order
4. Provide a terminal-based user interface for data operations
5. Support data transformation workflows
6. Enable cataloging of data models and sources
7. Facilitate sharing of data assets across teams

## Goals
- Simplify data transformation processes
- Improve data discoverability through cataloging
- Enhance collaboration among data teams
- Provide a robust CLI tool for data operations
- Support SQL as the primary language for data transformation
- Automate dependency management between models
- Create a lightweight alternative to dbt

## Target Users
- Data engineers
- Data analysts
- Data scientists
- Database administrators
- Other data team members working with SQL and data transformation
