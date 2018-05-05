function [select_methods, compare_methods] = get_folder_conditions(folder)
% find how many s and c's there are, to create matrix for comparing

if ~exist('folder','var')
   d = dir('./s*');
else
   d = strcat(folder,'/s*');
end


select_methods = [];
compare_methods = [];

for f = d'
    [s, c] = folder_name_to_nums(f.name)
    select_methods = [select_methods s];
    compare_methods = [compare_methods c];
end
select_methods = unique(select_methods);
compare_methods = unique(compare_methods);
