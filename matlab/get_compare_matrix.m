csf = 'eff_len' %compare_stat_folder
stat = 'ave';

[ss, cs] =  get_folder_conditions;

res = zeros(length(ss), length(cs))

if ~exist('folder','var')
   d = dir('./s*');
else
   d = strcat(folder,'/s*');
end

for f = d'
    fname = strcat(f.name, '/', csf, '/', stat, '.txt')
    [av, sd] = plot_distr(fname)
    
    [s, c] = folder_name_to_nums(f.name);
    si = find(ss==s);
    ci = find(cs==c);
    res(si,ci) = av;
end
res