function [s, c] = folder_name_to_nums(fname)

parts = strsplit(fname, '_');
s = char(parts(1));
s = str2double(s(2:end));
c = char(parts(2));
c = str2double(c(2:end));
