function [num1, num2] = folder_name_to_nums(fname, dim1, dim2)
parts = strsplit(fname, '_');
num1 = str2double(parts(dim1));
num2 = str2double(parts(dim2));
